use crate::{BinKey, CacheModel, Error, Known};
use alloc::{
    collections::BTreeSet,
    format,
    string::{String, ToString},
    vec::Vec,
};

/// Deliberately small request profile: one target/codestream, positive frame
/// and region, explicit component indices, byte cache models and no channels.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub target: String,
    pub tid: String,
    pub frame: [u32; 2],
    pub offset: [u32; 2],
    pub size: [u32; 2],
    pub components: Vec<u16>,
    pub layers: Option<u32>,
    pub max_length: u64,
    pub extended: bool,
    pub model: CacheModel,
}
fn number(s: &str) -> Result<u64, Error> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::Malformed);
    }
    s.parse().map_err(|_| Error::Limit)
}
fn pair(s: &str) -> Result<[u32; 2], Error> {
    let (a, b) = s.split_once(',').ok_or(Error::Malformed)?;
    Ok([
        u32::try_from(number(a)?).map_err(|_| Error::Limit)?,
        u32::try_from(number(b)?).map_err(|_| Error::Limit)?,
    ])
}
fn decode(s: &str) -> Result<String, Error> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] == b'%' {
            let hex = bytes.get(at + 1..at + 3).ok_or(Error::Malformed)?;
            let mut n = 0;
            for b in hex {
                n = n * 16
                    + match b {
                        b'0'..=b'9' => b - b'0',
                        b'a'..=b'f' => b - b'a' + 10,
                        b'A'..=b'F' => b - b'A' + 10,
                        _ => return Err(Error::Malformed),
                    };
            }
            out.push(n);
            at += 3;
        } else {
            if bytes[at] == b'+' {
                return Err(Error::Unsupported);
            }
            out.push(bytes[at]);
            at += 1;
        }
    }
    String::from_utf8(out).map_err(|_| Error::Malformed)
}
fn encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
pub fn parse_model(s: &str, max_bins: usize) -> Result<CacheModel, Error> {
    let mut model = CacheModel::new();
    if s.is_empty() {
        return Ok(model);
    }
    for item in s.split(',') {
        if model.len() >= max_bins {
            return Err(Error::Limit);
        }
        let (name, known) = match item.split_once(':') {
            Some((name, amount)) => (name, Known::Prefix(number(amount)?)),
            None => (item, Known::Complete),
        };
        let key = if name == "Hm" {
            BinKey::new(6, 0)?
        } else if let Some(id) = name.strip_prefix('H') {
            BinKey::new(2, number(id)?)?
        } else if let Some(id) = name.strip_prefix('P') {
            BinKey::new(0, number(id)?)?
        } else if let Some(id) = name.strip_prefix('M') {
            BinKey::new(8, number(id)?)?
        } else {
            return Err(Error::Unsupported);
        };
        if model.insert(key, known).is_some() {
            return Err(Error::Malformed);
        }
    }
    Ok(model)
}
pub fn format_model(model: &CacheModel) -> Result<String, Error> {
    let mut out = String::new();
    for (key, known) in model {
        if !out.is_empty() {
            out.push(',');
        }
        match key.class {
            6 if key.id == 0 => out.push_str("Hm"),
            2 => out.push_str(&format!("H{}", key.id)),
            0 => out.push_str(&format!("P{}", key.id)),
            8 => out.push_str(&format!("M{}", key.id)),
            _ => return Err(Error::Unsupported),
        }
        if let Known::Prefix(n) = known {
            out.push_str(&format!(":{n}"));
        }
    }
    Ok(out)
}
impl Request {
    pub fn validate(&self) -> Result<(), Error> {
        if self.target.is_empty()
            || self.target.len() > 255
            || !self
                .target
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-._".contains(&b))
        {
            return Err(Error::Unsupported);
        }
        validate_tid(&self.tid)?;
        if self.frame.contains(&0)
            || self.size.contains(&0)
            || self.max_length == 0
            || self.layers == Some(0)
            || self.components.is_empty()
        {
            return Err(Error::Malformed);
        }
        for axis in 0..2 {
            if self.offset[axis]
                .checked_add(self.size[axis])
                .is_none_or(|end| end > self.frame[axis])
            {
                return Err(Error::Malformed);
            }
        }
        if self.components.len() > 16384 || self.components.windows(2).any(|w| w[0] >= w[1]) {
            return Err(Error::Malformed);
        }
        format_model(&self.model)?;
        Ok(())
    }
    /// Query bytes only, without `?`. Unknown fields, duplicates, channels,
    /// wildcards, subtractive models and optional variants fail explicitly.
    pub fn parse(
        query: &str,
        max_query_bytes: usize,
        max_model_bins: usize,
    ) -> Result<Self, Error> {
        if query.len() > max_query_bytes {
            return Err(Error::Limit);
        }
        let mut fields = alloc::collections::BTreeMap::new();
        for field in query.split('&') {
            let (name, value) = field.split_once('=').ok_or(Error::Malformed)?;
            let name = decode(name)?;
            if ![
                "target", "tid", "fsiz", "roff", "rsiz", "comps", "layers", "len", "type", "model",
            ]
            .contains(&name.as_str())
            {
                return Err(Error::Unsupported);
            }
            if fields.insert(name, decode(value)?).is_some() {
                return Err(Error::Malformed);
            }
        }
        let get = |name: &str| fields.get(name).map(String::as_str).ok_or(Error::Malformed);
        let mut seen = BTreeSet::new();
        let components = get("comps")?
            .split(',')
            .map(|s| {
                let v = u16::try_from(number(s)?).map_err(|_| Error::Limit)?;
                if !seen.insert(v) {
                    return Err(Error::Malformed);
                }
                Ok(v)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let extended = match get("type")? {
            "jpp-stream" => false,
            "jpp-stream;ptype=ext" => true,
            _ => return Err(Error::Unsupported),
        };
        let request = Self {
            target: get("target")?.to_string(),
            tid: get("tid")?.to_string(),
            frame: pair(get("fsiz")?)?,
            offset: pair(get("roff")?)?,
            size: pair(get("rsiz")?)?,
            components,
            layers: fields
                .get("layers")
                .map(|s| number(s).and_then(|n| u32::try_from(n).map_err(|_| Error::Limit)))
                .transpose()?,
            max_length: number(get("len")?)?,
            extended,
            model: fields
                .get("model")
                .map(|s| parse_model(s, max_model_bins))
                .transpose()?
                .unwrap_or_default(),
        };
        request.validate()?;
        Ok(request)
    }
    pub fn query(&self) -> Result<String, Error> {
        self.validate()?;
        let comps = self
            .components
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let mut fields = alloc::vec![
            ("target", self.target.clone()),
            ("tid", self.tid.clone()),
            ("fsiz", format!("{},{}", self.frame[0], self.frame[1])),
            ("roff", format!("{},{}", self.offset[0], self.offset[1])),
            ("rsiz", format!("{},{}", self.size[0], self.size[1])),
            ("comps", comps),
            ("len", self.max_length.to_string()),
            (
                "type",
                if self.extended {
                    "jpp-stream;ptype=ext"
                } else {
                    "jpp-stream"
                }
                .to_string()
            ),
        ];
        if let Some(layers) = self.layers {
            fields.push(("layers", layers.to_string()));
        }
        if !self.model.is_empty() {
            fields.push(("model", format_model(&self.model)?));
        }
        Ok(fields
            .into_iter()
            .map(|(k, v)| format!("{k}={}", encode(&v)))
            .collect::<Vec<_>>()
            .join("&"))
    }
    /// A mismatching target identity invalidates every cache assertion.
    pub fn model_for_identity(&self, actual_tid: &str) -> CacheModel {
        if self.tid != "0" && actual_tid != "0" && self.tid == actual_tid {
            self.model.clone()
        } else {
            CacheModel::new()
        }
    }
}
fn validate_tid(tid: &str) -> Result<(), Error> {
    if tid.is_empty() || tid.len() > 255 || !tid.bytes().all(|b| (33..=126).contains(&b)) {
        Err(Error::Identity)
    } else {
        Ok(())
    }
}
/// Selected response fields. Values use the same syntax as request fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseFields {
    pub tid: String,
    pub frame: [u32; 2],
    pub offset: [u32; 2],
    pub size: [u32; 2],
}
impl ResponseFields {
    pub fn parse(tid: &str, fsiz: &str, roff: &str, rsiz: &str) -> Result<Self, Error> {
        validate_tid(tid)?;
        let value = Self {
            tid: tid.to_string(),
            frame: pair(fsiz)?,
            offset: pair(roff)?,
            size: pair(rsiz)?,
        };
        if value.frame.contains(&0) || value.size.contains(&0) {
            return Err(Error::Malformed);
        }
        for axis in 0..2 {
            if value.offset[axis]
                .checked_add(value.size[axis])
                .is_none_or(|end| end > value.frame[axis])
            {
                return Err(Error::Malformed);
            }
        }
        Ok(value)
    }
    pub fn headers(&self) -> Result<[(String, String); 4], Error> {
        let pairs = [self.frame, self.offset, self.size].map(|v| format!("{},{}", v[0], v[1]));
        Self::parse(&self.tid, &pairs[0], &pairs[1], &pairs[2])?;
        Ok([
            ("JPIP-tid".into(), self.tid.clone()),
            ("JPIP-fsiz".into(), pairs[0].clone()),
            ("JPIP-roff".into(), pairs[1].clone()),
            ("JPIP-rsiz".into(), pairs[2].clone()),
        ])
    }
}
