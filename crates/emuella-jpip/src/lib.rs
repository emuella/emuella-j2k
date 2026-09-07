//! Selected stateless HTTP JPP profile; see the package README for boundaries.
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod cache;
mod delivery;
mod framing;
mod request;
pub use cache::*;
pub use delivery::*;
pub use framing::*;
pub use request::*;

/// Malformed input, unsupported selected-profile behaviour, or a resource limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Malformed,
    Unsupported,
    Limit,
    Truncated,
    Conflict,
    Identity,
    Source,
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "JPIP {:?}", self)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Normalised JPP bin identity. Extended precinct class 1 shares class 0 storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BinKey {
    pub class: u64,
    pub id: u64,
}
impl BinKey {
    pub fn new(class: u64, id: u64) -> Result<Self, Error> {
        match class {
            0 | 1 => Ok(Self { class: 0, id }),
            2 | 8 => Ok(Self { class, id }),
            6 if id == 0 => Ok(Self { class, id }),
            _ => Err(Error::Unsupported),
        }
    }
}

/// Annex A.3.2.1 precinct identity for a single codestream.
pub fn precinct_id(
    tile: u64,
    tiles: u64,
    component: u64,
    components: u64,
    sequence: u64,
) -> Result<u64, Error> {
    if tiles == 0 || components == 0 || tile >= tiles || component >= components {
        return Err(Error::Malformed);
    }
    sequence
        .checked_mul(components)
        .and_then(|v| v.checked_add(component))
        .and_then(|v| v.checked_mul(tiles))
        .and_then(|v| v.checked_add(tile))
        .ok_or(Error::Limit)
}
