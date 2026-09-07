use crate::{BinKey, Error};
use alloc::vec::Vec;

/// One delivered fragment. `final_bin` applies to the end of this fragment only.
#[derive(Clone, Copy, Debug)]
pub struct DataMessage<'a> {
    pub key: BinKey,
    pub offset: u64,
    pub final_bin: bool,
    pub auxiliary: Option<u64>,
    pub bytes: &'a [u8],
}
#[derive(Clone, Copy, Debug)]
pub enum Event<'a> {
    Data(DataMessage<'a>),
    End { reason: u8 },
}
#[derive(Clone, Copy)]
struct Header {
    class: u64,
    csn: u64,
    id: u64,
    offset: u64,
    remaining: u64,
    final_bin: bool,
    auxiliary: Option<u64>,
    reason: Option<u8>,
}
/// Incremental response parser. A parser belongs to one HTTP response, while a
/// cache may outlive many responses. On any error discard this parser.
pub struct Decoder {
    header: Vec<u8>,
    pending: Option<Header>,
    class: u64,
    csn: u64,
    max_body: u64,
    ended: bool,
    failed: bool,
}
impl Decoder {
    pub fn new(max_body: u64) -> Self {
        Self {
            header: Vec::new(),
            pending: None,
            class: 0,
            csn: 0,
            max_body,
            ended: false,
            failed: false,
        }
    }
    pub fn push(
        &mut self,
        input: &[u8],
        mut emit: impl FnMut(Event<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if self.failed {
            return Err(Error::Malformed);
        }
        let result = self.consume(input, &mut emit);
        if result.is_err() {
            self.failed = true;
        }
        result
    }
    fn consume(
        &mut self,
        mut input: &[u8],
        emit: &mut impl FnMut(Event<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        while !input.is_empty() || self.pending.is_some_and(|p| p.remaining == 0) {
            if self.ended {
                return Err(Error::Malformed);
            }
            if let Some(mut h) = self.pending.take() {
                let amount = core::cmp::min(h.remaining, input.len() as u64) as usize;
                let bytes = &input[..amount];
                input = &input[amount..];
                h.remaining -= amount as u64;
                if h.reason.is_none()
                    && (h.csn == 0 || h.class == 8)
                    && let Ok(key) = BinKey::new(h.class, h.id)
                {
                    emit(Event::Data(DataMessage {
                        key,
                        offset: h.offset,
                        final_bin: h.final_bin && h.remaining == 0,
                        auxiliary: if h.remaining == 0 { h.auxiliary } else { None },
                        bytes,
                    }))?;
                }
                h.offset = h.offset.checked_add(amount as u64).ok_or(Error::Limit)?;
                if h.remaining == 0 {
                    if let Some(reason) = h.reason {
                        self.ended = true;
                        emit(Event::End { reason })?;
                    }
                } else {
                    self.pending = Some(h);
                }
            } else {
                self.header.push(input[0]);
                input = &input[1..];
                if self.header.len() > 64 {
                    return Err(Error::Limit);
                }
                if let Some(h) = parse_header(&self.header, self.class, self.csn)? {
                    if h.remaining > self.max_body {
                        return Err(Error::Limit);
                    }
                    h.offset.checked_add(h.remaining).ok_or(Error::Limit)?;
                    if h.reason.is_none() {
                        self.class = h.class;
                        self.csn = h.csn;
                    }
                    self.header.clear();
                    self.pending = Some(h);
                }
            }
        }
        Ok(())
    }
    /// An EOR and an exact framing boundary are mandatory for a complete response.
    pub fn finish(&self) -> Result<(), Error> {
        if self.failed {
            Err(Error::Malformed)
        } else if self.ended && self.header.is_empty() && self.pending.is_none() {
            Ok(())
        } else {
            Err(Error::Truncated)
        }
    }
}
fn vbas(bytes: &[u8], at: &mut usize) -> Result<Option<u64>, Error> {
    let mut value = 0u64;
    for count in 0..10 {
        let Some(&b) = bytes.get(*at) else {
            return Ok(None);
        };
        *at += 1;
        value = value
            .checked_mul(128)
            .and_then(|v| v.checked_add((b & 127) as u64))
            .ok_or(Error::Limit)?;
        if b & 128 == 0 {
            return Ok(Some(value));
        }
        if count == 9 {
            return Err(Error::Limit);
        }
    }
    Err(Error::Limit)
}
fn parse_header(
    bytes: &[u8],
    previous_class: u64,
    previous_csn: u64,
) -> Result<Option<Header>, Error> {
    let first = bytes[0];
    let mut at = 1;
    if first == 0 {
        let Some(&reason) = bytes.get(at) else {
            return Ok(None);
        };
        at += 1;
        let Some(length) = vbas(bytes, &mut at)? else {
            return Ok(None);
        };
        return Ok(Some(Header {
            reason: Some(reason),
            remaining: length,
            class: 0,
            csn: 0,
            id: 0,
            offset: 0,
            final_bin: false,
            auxiliary: None,
        }));
    }
    let presence = (first >> 5) & 3;
    if presence == 0 {
        return Err(Error::Malformed);
    }
    let mut id = (first & 15) as u64;
    let mut b = first;
    while b & 128 != 0 {
        let Some(&next) = bytes.get(at) else {
            return Ok(None);
        };
        at += 1;
        b = next;
        id = id
            .checked_mul(128)
            .and_then(|v| v.checked_add((b & 127) as u64))
            .ok_or(Error::Limit)?;
        if at > 10 {
            return Err(Error::Limit);
        }
    }
    let class = if presence >= 2 {
        let Some(v) = vbas(bytes, &mut at)? else {
            return Ok(None);
        };
        v
    } else {
        previous_class
    };
    let csn = if presence == 3 {
        let Some(v) = vbas(bytes, &mut at)? else {
            return Ok(None);
        };
        v
    } else {
        previous_csn
    };
    let Some(offset) = vbas(bytes, &mut at)? else {
        return Ok(None);
    };
    let Some(remaining) = vbas(bytes, &mut at)? else {
        return Ok(None);
    };
    let auxiliary = if class & 1 != 0 {
        let Some(v) = vbas(bytes, &mut at)? else {
            return Ok(None);
        };
        Some(v)
    } else {
        None
    };
    Ok(Some(Header {
        class,
        csn,
        id,
        offset,
        remaining,
        final_bin: first & 16 != 0,
        auxiliary,
        reason: None,
    }))
}
fn put_vbas(value: u64, out: &mut Vec<u8>) {
    let mut groups = [0u8; 10];
    let mut n = groups.len();
    let mut value = value;
    loop {
        n -= 1;
        groups[n] = (value & 127) as u8;
        value >>= 7;
        if value == 0 {
            break;
        }
    }
    let end = groups.len() - 1;
    for (i, &b) in groups.iter().enumerate().skip(n) {
        out.push(b | if i == end { 0 } else { 128 });
    }
}
/// Explicit class and codestream identifiers make each generated message
/// independently parseable. Only codestream zero is emitted.
pub fn encode_message(message: DataMessage<'_>) -> Result<Vec<u8>, Error> {
    if BinKey::new(message.key.class, message.key.id)? != message.key {
        return Err(Error::Malformed);
    }
    message
        .offset
        .checked_add(message.bytes.len() as u64)
        .ok_or(Error::Limit)?;
    if message.auxiliary.is_some() && message.key.class != 0 {
        return Err(Error::Unsupported);
    }
    let mut out = Vec::new();
    let mut trailing = [0u8; 9];
    let mut at = trailing.len();
    let mut id = message.key.id;
    while id > 15 {
        at -= 1;
        trailing[at] = (id & 127) as u8;
        id >>= 7;
    }
    out.push(
        0x60 | if message.final_bin { 16 } else { 0 }
            | id as u8
            | if at < trailing.len() { 128 } else { 0 },
    );
    for (i, &b) in trailing.iter().enumerate().skip(at) {
        out.push(b | if i + 1 < trailing.len() { 128 } else { 0 });
    }
    put_vbas(
        message.key.class + u64::from(message.auxiliary.is_some()),
        &mut out,
    );
    put_vbas(0, &mut out);
    put_vbas(message.offset, &mut out);
    put_vbas(message.bytes.len() as u64, &mut out);
    if let Some(auxiliary) = message.auxiliary {
        put_vbas(auxiliary, &mut out);
    }
    out.extend_from_slice(message.bytes);
    Ok(out)
}
/// Empty EOR body. Reasons are protocol values; the selected server emits
/// window completion (2) or byte limit (4).
pub fn encode_end(reason: u8) -> [u8; 3] {
    [0, reason, 0]
}
