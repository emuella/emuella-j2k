use crate::{BinKey, CacheModel, DataMessage, Error, Known, encode_end, encode_message};

/// An adapter exposes virtual bins without constructing a whole-source copy.
/// `read` must fill exactly the requested range or return an error.
pub trait BinSource {
    fn length(&self, key: BinKey) -> Result<u64, Error>;
    fn read(&mut self, key: BinKey, offset: u64, output: &mut [u8]) -> Result<(), Error>;
    /// Extended precinct messages require an accurate completed-packet count
    /// at this prefix. For each key, return either None at every prefix, or
    /// Some with a monotonically increasing count. None emits normal messages.
    fn completed_packets(&self, _key: BinKey, _prefix: u64) -> Option<u64> {
        None
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Demand {
    pub key: BinKey,
    /// None requests the entire bin; Some requests at most this many bytes.
    pub prefix: Option<u64>,
}
#[derive(Clone, Copy, Debug)]
pub struct DeliveryLimits {
    /// JPIP len counts normal headers and bodies, excluding EOR.
    pub response_bytes: u64,
    pub message_bytes: usize,
    pub messages: usize,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeliveryStats {
    pub source_bytes: u64,
    pub wire_bytes: u64,
    pub messages: usize,
    pub window_complete: bool,
}
fn groups(mut n: u64) -> u64 {
    let mut count = 1;
    while n >= 128 {
        n >>= 7;
        count += 1;
    }
    count
}
/// Stream a selected ordered set of bin prefixes. Geometry, component-transform
/// dependencies and quality-prefix selection belong to the codec adapter.
/// The model must already be invalidated on target-id mismatch. A failed write
/// aborts the response; retry with a fresh model from the client's actual cache.
pub fn deliver(
    source: &mut impl BinSource,
    demands: &[Demand],
    model: &CacheModel,
    limits: DeliveryLimits,
    extended: bool,
    mut write: impl FnMut(&[u8]) -> Result<(), Error>,
) -> Result<DeliveryStats, Error> {
    if limits.message_bytes == 0 || limits.messages == 0 {
        return Err(Error::Limit);
    }
    let mut stats = DeliveryStats::default();
    let mut left = limits.response_bytes;
    let mut reason = 2;
    let mut seen = alloc::collections::BTreeSet::new();
    'demand: for demand in demands {
        if !seen.insert(demand.key) {
            return Err(Error::Malformed);
        }
        let length = source.length(demand.key)?;
        let wanted = demand.prefix.unwrap_or(length).min(length);
        let mut start = match model.get(&demand.key) {
            Some(Known::Complete) => continue,
            Some(Known::Prefix(n)) => (*n).min(length),
            None => 0,
        };
        if start >= wanted && wanted < length {
            continue;
        }
        loop {
            if stats.messages >= limits.messages {
                reason = 7;
                break 'demand;
            }
            let maximum = (wanted.saturating_sub(start))
                .min(limits.message_bytes as u64)
                .min(left);
            // Header size can grow at VBAS boundaries. Binary search an exact
            // payload bound before reading any source bytes.
            let header_size = |n: u64| -> Result<u64, Error> {
                let auxiliary = if extended && demand.key.class == 0 {
                    source.completed_packets(demand.key, start + n)
                } else {
                    None
                };
                let header = encode_message(DataMessage {
                    key: demand.key,
                    offset: start,
                    final_bin: start + n == length,
                    auxiliary,
                    bytes: &[],
                })?;
                Ok(header.len() as u64 + groups(n) - 1)
            };
            // Packet counts are monotonic in a well-formed source, so header
            // lengths cannot shrink as the candidate prefix increases.
            let mut low = 0;
            let mut high = maximum;
            if header_size(0)? > left {
                reason = 4;
                break 'demand;
            }
            while low < high {
                let middle = low + (high - low).div_ceil(2);
                if middle
                    .checked_add(header_size(middle)?)
                    .is_some_and(|n| n <= left)
                {
                    low = middle;
                } else {
                    high = middle - 1;
                }
            }
            let count = usize::try_from(low).map_err(|_| Error::Limit)?;
            if count == 0 && start < wanted {
                reason = 4;
                break 'demand;
            }
            let mut payload = alloc::vec![0; count];
            if count > 0 {
                source.read(demand.key, start, &mut payload)?;
            }
            let end = start + count as u64;
            let auxiliary = if extended && demand.key.class == 0 {
                source.completed_packets(demand.key, end)
            } else {
                None
            };
            let message = encode_message(DataMessage {
                key: demand.key,
                offset: start,
                final_bin: end == length,
                auxiliary,
                bytes: &payload,
            })?;
            if message.len() as u64 > left {
                return Err(Error::Source);
            }
            write(&message)?;
            left -= message.len() as u64;
            stats.source_bytes += count as u64;
            stats.wire_bytes += message.len() as u64;
            stats.messages += 1;
            start = end;
            if start >= wanted {
                continue 'demand;
            }
        }
    }
    stats.window_complete = reason == 2;
    let eor = encode_end(reason);
    write(&eor)?;
    stats.wire_bytes += eor.len() as u64;
    Ok(stats)
}
