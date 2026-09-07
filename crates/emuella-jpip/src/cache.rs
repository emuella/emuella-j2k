use crate::{BinKey, DataMessage, Error};
use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[derive(Clone, Copy, Debug)]
pub struct CacheLimits {
    pub bytes: usize,
    pub bins: usize,
    pub ranges_per_bin: usize,
    pub max_bin_length: u64,
}
#[derive(Clone, Debug)]
struct Segment {
    offset: u64,
    bytes: Vec<u8>,
}
#[derive(Clone, Debug, Default)]
struct Bin {
    segments: Vec<Segment>,
    final_length: Option<u64>,
    used: u64,
}
impl Bin {
    fn bytes(&self) -> usize {
        self.segments.iter().map(|s| s.bytes.len()).sum()
    }
    fn prefix(&self) -> u64 {
        self.segments
            .first()
            .filter(|s| s.offset == 0)
            .map_or(0, |s| s.bytes.len() as u64)
    }
}
/// The server interprets each model against an initially empty cache.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Known {
    Prefix(u64),
    Complete,
}
pub type CacheModel = BTreeMap<BinKey, Known>;

/// Sparse compressed cache with whole-bin LRU eviction. Its payload byte budget
/// is separate from bounded bin/range metadata. Caller owns response identity
/// validation before passing any response bytes to `insert`.
pub struct Cache {
    identity: Option<String>,
    bins: BTreeMap<BinKey, Bin>,
    limits: CacheLimits,
    clock: u64,
    bytes: usize,
    evictions: u64,
}
impl Cache {
    pub fn new(limits: CacheLimits) -> Self {
        Self {
            identity: None,
            bins: BTreeMap::new(),
            limits,
            clock: 0,
            bytes: 0,
            evictions: 0,
        }
    }
    /// Call before decoding each response. A changed identity discards all old
    /// bins. Identity `0` cannot establish reusable state and always resets it.
    pub fn bind_identity(&mut self, identity: &str) -> Result<bool, Error> {
        if identity.is_empty()
            || identity.len() > 255
            || !identity.bytes().all(|b| (33..=126).contains(&b))
        {
            return Err(Error::Identity);
        }
        let changed = identity == "0" || self.identity.as_deref() != Some(identity);
        if changed {
            self.bins.clear();
            self.bytes = 0;
        }
        self.identity = Some(String::from(identity));
        Ok(changed)
    }
    pub fn identity(&self) -> Option<&str> {
        self.identity.as_deref()
    }
    pub fn bytes(&self) -> usize {
        self.bytes
    }
    pub fn bin_count(&self) -> usize {
        self.bins.len()
    }
    /// Cumulative whole-bin explicit/LRU evictions; identity resets are excluded.
    pub fn eviction_count(&self) -> u64 {
        self.evictions
    }
    pub fn evict(&mut self, key: BinKey) -> bool {
        if let Some(bin) = self.bins.remove(&key) {
            self.bytes -= bin.bytes();
            self.evictions = self.evictions.saturating_add(1);
            true
        } else {
            false
        }
    }
    pub fn model(&self) -> CacheModel {
        if self.identity.as_deref() == Some("0") {
            return CacheModel::new();
        }
        self.bins
            .iter()
            .filter_map(|(&key, bin)| {
                let prefix = bin.prefix();
                if bin.final_length == Some(prefix) {
                    Some((key, Known::Complete))
                } else if prefix > 0 {
                    Some((key, Known::Prefix(prefix)))
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn is_complete(&self, key: BinKey) -> bool {
        self.bins
            .get(&key)
            .is_some_and(|b| b.final_length == Some(b.prefix()))
    }
    /// Read one contiguous cached range into caller storage; no partial writes.
    pub fn read(&mut self, key: BinKey, offset: u64, out: &mut [u8]) -> Result<(), Error> {
        let end = offset.checked_add(out.len() as u64).ok_or(Error::Limit)?;
        let bin = self.bins.get_mut(&key).ok_or(Error::Truncated)?;
        let segment = bin
            .segments
            .iter()
            .find(|s| s.offset <= offset && s.offset + s.bytes.len() as u64 >= end)
            .ok_or(Error::Truncated)?;
        let start = (offset - segment.offset) as usize;
        out.copy_from_slice(&segment.bytes[start..start + out.len()]);
        self.clock = self.clock.saturating_add(1);
        bin.used = self.clock;
        Ok(())
    }
    /// Overlap is accepted only when every overlapping byte agrees. Validation
    /// happens before mutation or eviction. An isolated final fragment records
    /// length but does not mark the bin complete or advertise holes.
    pub fn insert(&mut self, message: DataMessage<'_>) -> Result<(), Error> {
        if self.identity.is_none() {
            return Err(Error::Identity);
        }
        if BinKey::new(message.key.class, message.key.id)? != message.key {
            return Err(Error::Malformed);
        }
        let end = message
            .offset
            .checked_add(message.bytes.len() as u64)
            .ok_or(Error::Limit)?;
        if end > self.limits.max_bin_length || self.limits.bins == 0 {
            return Err(Error::Limit);
        }
        let old = self.bins.get(&message.key);
        let final_length = if message.final_bin {
            Some(end)
        } else {
            old.and_then(|b| b.final_length)
        };
        if let Some(bin) = old {
            if bin
                .final_length
                .is_some_and(|n| end > n || (message.final_bin && n != end))
            {
                return Err(Error::Conflict);
            }
            if message.final_bin
                && bin
                    .segments
                    .iter()
                    .any(|s| s.offset + s.bytes.len() as u64 > end)
            {
                return Err(Error::Conflict);
            }
            for s in &bin.segments {
                let a = core::cmp::max(s.offset, message.offset);
                let b = core::cmp::min(s.offset + s.bytes.len() as u64, end);
                if a < b
                    && s.bytes[(a - s.offset) as usize..(b - s.offset) as usize]
                        != message.bytes
                            [(a - message.offset) as usize..(b - message.offset) as usize]
                {
                    return Err(Error::Conflict);
                }
            }
        }
        let mut start = message.offset;
        let mut stop = end;
        let mut replaced = 0usize;
        let mut touched = 0usize;
        if let Some(bin) = old {
            for s in &bin.segments {
                let e = s.offset + s.bytes.len() as u64;
                if e >= start && s.offset <= stop {
                    start = core::cmp::min(start, s.offset);
                    stop = core::cmp::max(stop, e);
                    replaced += s.bytes.len();
                    touched += 1;
                }
            }
        }
        let combined = usize::try_from(stop - start).map_err(|_| Error::Limit)?;
        let old_bytes = old.map_or(0, Bin::bytes);
        let new_bytes = old_bytes
            .checked_sub(replaced)
            .and_then(|n| n.checked_add(combined))
            .ok_or(Error::Limit)?;
        let ranges = old.map_or(0, |b| b.segments.len()) - touched + usize::from(combined > 0);
        if new_bytes > self.limits.bytes || ranges > self.limits.ranges_per_bin {
            return Err(Error::Limit);
        }
        // At most one bounded merged interval is allocated; existing untouched
        // ranges remain in place. Temporary payload overhead is <= the byte cap.
        let mut merged = alloc::vec![0; combined];
        if let Some(bin) = old {
            for s in &bin.segments {
                if s.offset >= start && s.offset + s.bytes.len() as u64 <= stop {
                    let at = (s.offset - start) as usize;
                    merged[at..at + s.bytes.len()].copy_from_slice(&s.bytes);
                }
            }
        }
        let at = (message.offset - start) as usize;
        merged[at..at + message.bytes.len()].copy_from_slice(message.bytes);
        while self.bytes - old_bytes + new_bytes > self.limits.bytes
            || (!self.bins.contains_key(&message.key) && self.bins.len() >= self.limits.bins)
        {
            let victim = self
                .bins
                .iter()
                .filter(|(k, _)| **k != message.key)
                .min_by_key(|(_, b)| b.used)
                .map(|(&k, _)| k)
                .ok_or(Error::Limit)?;
            self.evict(victim);
        }
        self.clock = self.clock.saturating_add(1);
        let bin = self.bins.entry(message.key).or_default();
        bin.segments
            .retain(|s| !(s.offset >= start && s.offset + s.bytes.len() as u64 <= stop));
        if combined > 0 {
            bin.segments.push(Segment {
                offset: start,
                bytes: merged,
            });
        }
        bin.segments.sort_unstable_by_key(|s| s.offset);
        bin.final_length = final_length;
        bin.used = self.clock;
        self.bytes = self.bytes - old_bytes + new_bytes;
        Ok(())
    }
}
