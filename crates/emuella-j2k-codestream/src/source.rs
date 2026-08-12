//! Immutable positioned-read sources for source-backed prepared decoding.

use alloc::string::String;
use core::fmt;
use core::sync::atomic::{AtomicU64, Ordering};

/// Stable class for a positioned source failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceErrorKind {
    OutOfRange,
    ShortRead,
    Io,
}

/// Structured source failure with logical `u64` range provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceError {
    pub kind: SourceErrorKind,
    pub offset: u64,
    pub requested: u64,
    pub available: u64,
    pub message: String,
}

impl SourceError {
    fn range(kind: SourceErrorKind, offset: u64, requested: u64, available: u64) -> Self {
        Self {
            kind,
            offset,
            requested,
            available,
            message: String::from("positioned source range is unavailable"),
        }
    }

    #[cfg(feature = "std")]
    fn io(offset: u64, requested: u64, available: u64, error: std::io::Error) -> Self {
        Self {
            kind: if error.kind() == std::io::ErrorKind::UnexpectedEof {
                SourceErrorKind::ShortRead
            } else {
                SourceErrorKind::Io
            },
            offset,
            requested,
            available,
            message: error.to_string(),
        }
    }
}

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "source {:?} at byte {}: requested {}, available {} ({})",
            self.kind, self.offset, self.requested, self.available, self.message
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SourceError {}

/// Physical access counters. Logical codec skip counters remain separate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SourceMetrics {
    pub source_bytes_requested: u64,
    pub source_bytes_returned: u64,
    pub source_read_operations: u64,
    pub source_ranges_coalesced: u64,
    pub source_cache_hits: u64,
    pub source_cache_misses: u64,
    pub largest_source_read: u64,
    /// Nearest-rank median read size, reported as the upper bound of a
    /// power-of-two histogram bucket.
    pub median_source_read_upper_bound: u64,
    /// Nearest-rank p95 read size, reported as the upper bound of a
    /// power-of-two histogram bucket.
    pub p95_source_read_upper_bound: u64,
    pub tile_part_bytes_not_read: u64,
    pub packet_body_bytes_not_read: u64,
}

/// Immutable cursor-independent byte source.
///
/// Implementations must keep bytes stable for the lifetime of every prepared
/// plan bound to the source. Mutating or replacing backing storage while a plan
/// exists violates the contract.
pub trait CodestreamSource: Send + Sync {
    fn len(&self) -> Result<u64, SourceError>;

    fn is_empty(&self) -> Result<bool, SourceError> {
        self.len().map(|length| length == 0)
    }

    fn read_exact_at(&self, offset: u64, destination: &mut [u8]) -> Result<(), SourceError>;

    fn metrics(&self) -> Option<SourceMetrics> {
        None
    }

    #[doc(hidden)]
    fn record_coalesced_range(&self) {}

    #[doc(hidden)]
    fn record_bytes_not_read(&self, _tile_part_bytes: u64, _packet_body_bytes: u64) {}
}

impl<T: CodestreamSource + ?Sized> CodestreamSource for &T {
    fn len(&self) -> Result<u64, SourceError> {
        (**self).len()
    }

    fn read_exact_at(&self, offset: u64, destination: &mut [u8]) -> Result<(), SourceError> {
        (**self).read_exact_at(offset, destination)
    }

    fn metrics(&self) -> Option<SourceMetrics> {
        (**self).metrics()
    }

    fn record_coalesced_range(&self) {
        (**self).record_coalesced_range();
    }

    fn record_bytes_not_read(&self, tile_part_bytes: u64, packet_body_bytes: u64) {
        (**self).record_bytes_not_read(tile_part_bytes, packet_body_bytes);
    }
}

/// Zero-copy adapter over an immutable byte slice.
#[derive(Debug, Clone, Copy)]
pub struct SliceSource<'a> {
    bytes: &'a [u8],
}

impl<'a> SliceSource<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    pub const fn as_slice(&self) -> &'a [u8] {
        self.bytes
    }
}

impl CodestreamSource for SliceSource<'_> {
    fn len(&self) -> Result<u64, SourceError> {
        Ok(self.bytes.len() as u64)
    }

    fn read_exact_at(&self, offset: u64, destination: &mut [u8]) -> Result<(), SourceError> {
        let requested = destination.len() as u64;
        let available = (self.bytes.len() as u64).saturating_sub(offset);
        let start = usize::try_from(offset).map_err(|_| {
            SourceError::range(SourceErrorKind::OutOfRange, offset, requested, available)
        })?;
        let end = start.checked_add(destination.len()).ok_or_else(|| {
            SourceError::range(SourceErrorKind::OutOfRange, offset, requested, available)
        })?;
        let source = self.bytes.get(start..end).ok_or_else(|| {
            SourceError::range(SourceErrorKind::ShortRead, offset, requested, available)
        })?;
        destination.copy_from_slice(source);
        Ok(())
    }
}

/// One bounded range retained by [`BufferedSourceCursor`].
#[derive(Debug, Default)]
pub struct SourceWindow {
    pub source_offset: u64,
    pub bytes: alloc::vec::Vec<u8>,
}

/// Sequential cursor that coalesces byte/header reads into bounded positioned
/// source windows. It owns all mutable cursor state and is never stored in an
/// immutable prepared plan.
pub struct BufferedSourceCursor<'a> {
    source: &'a dyn CodestreamSource,
    window: SourceWindow,
    logical_position: u64,
    window_capacity: usize,
}

impl<'a> BufferedSourceCursor<'a> {
    pub fn new(
        source: &'a dyn CodestreamSource,
        logical_position: u64,
        window_capacity: usize,
    ) -> Result<Self, SourceError> {
        let len = source.len()?;
        if window_capacity == 0 || logical_position > len {
            return Err(SourceError::range(
                SourceErrorKind::OutOfRange,
                logical_position,
                0,
                len.saturating_sub(logical_position),
            ));
        }
        Ok(Self {
            source,
            window: SourceWindow::default(),
            logical_position,
            window_capacity,
        })
    }

    pub const fn position(&self) -> u64 {
        self.logical_position
    }

    pub fn seek(&mut self, logical_position: u64) -> Result<(), SourceError> {
        let len = self.source.len()?;
        if logical_position > len {
            return Err(SourceError::range(
                SourceErrorKind::OutOfRange,
                logical_position,
                0,
                0,
            ));
        }
        self.logical_position = logical_position;
        Ok(())
    }

    pub fn read_u8(&mut self) -> Result<u8, SourceError> {
        let mut byte = [0_u8; 1];
        self.read_exact(&mut byte)?;
        Ok(byte[0])
    }

    pub fn read_exact(&mut self, destination: &mut [u8]) -> Result<(), SourceError> {
        if destination.is_empty() {
            return Ok(());
        }
        if destination.len() > self.window_capacity {
            self.source
                .read_exact_at(self.logical_position, destination)?;
            self.logical_position = self
                .logical_position
                .checked_add(destination.len() as u64)
                .ok_or_else(|| {
                    SourceError::range(
                        SourceErrorKind::OutOfRange,
                        self.logical_position,
                        destination.len() as u64,
                        0,
                    )
                })?;
            return Ok(());
        }

        let mut written = 0_usize;
        while written < destination.len() {
            let local = self
                .logical_position
                .checked_sub(self.window.source_offset)
                .and_then(|offset| usize::try_from(offset).ok());
            let available = local
                .and_then(|offset| self.window.bytes.len().checked_sub(offset))
                .unwrap_or(0);
            if available == 0 {
                self.refill()?;
                continue;
            }
            self.source.record_coalesced_range();
            let take = available.min(destination.len() - written);
            let local = local.ok_or_else(|| {
                SourceError::range(
                    SourceErrorKind::OutOfRange,
                    self.logical_position,
                    destination.len() as u64,
                    0,
                )
            })?;
            destination[written..written + take]
                .copy_from_slice(&self.window.bytes[local..local + take]);
            written += take;
            self.logical_position += take as u64;
        }
        Ok(())
    }

    fn refill(&mut self) -> Result<(), SourceError> {
        let len = self.source.len()?;
        let available = len.saturating_sub(self.logical_position);
        let read_len =
            usize::try_from((self.window_capacity as u64).min(available)).map_err(|_| {
                SourceError::range(
                    SourceErrorKind::OutOfRange,
                    self.logical_position,
                    self.window_capacity as u64,
                    available,
                )
            })?;
        if read_len == 0 {
            return Err(SourceError::range(
                SourceErrorKind::ShortRead,
                self.logical_position,
                1,
                0,
            ));
        }
        self.window.source_offset = self.logical_position;
        self.window.bytes.resize(read_len, 0);
        self.source
            .read_exact_at(self.logical_position, &mut self.window.bytes)
    }
}

/// Bounded logical source whose offset zero maps to `base_offset` in `inner`.
#[derive(Debug)]
pub struct SubrangeSource<S> {
    inner: S,
    base_offset: u64,
    len: u64,
}

impl<S: CodestreamSource> SubrangeSource<S> {
    pub fn new(inner: S, base_offset: u64, len: u64) -> Result<Self, SourceError> {
        let source_len = inner.len()?;
        let end = base_offset
            .checked_add(len)
            .ok_or_else(|| SourceError::range(SourceErrorKind::OutOfRange, base_offset, len, 0))?;
        if end > source_len {
            return Err(SourceError::range(
                SourceErrorKind::ShortRead,
                base_offset,
                len,
                source_len.saturating_sub(base_offset),
            ));
        }
        Ok(Self {
            inner,
            base_offset,
            len,
        })
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<S: CodestreamSource> CodestreamSource for SubrangeSource<S> {
    fn len(&self) -> Result<u64, SourceError> {
        Ok(self.len)
    }

    fn read_exact_at(&self, offset: u64, destination: &mut [u8]) -> Result<(), SourceError> {
        let requested = destination.len() as u64;
        let available = self.len.saturating_sub(offset);
        let end = offset.checked_add(requested).ok_or_else(|| {
            SourceError::range(SourceErrorKind::OutOfRange, offset, requested, available)
        })?;
        if end > self.len {
            return Err(SourceError::range(
                SourceErrorKind::ShortRead,
                offset,
                requested,
                available,
            ));
        }
        self.inner
            .read_exact_at(self.base_offset + offset, destination)
            .map_err(|mut error| {
                error.offset = error.offset.saturating_sub(self.base_offset);
                error
            })
    }

    fn metrics(&self) -> Option<SourceMetrics> {
        self.inner.metrics()
    }

    fn record_coalesced_range(&self) {
        self.inner.record_coalesced_range();
    }

    fn record_bytes_not_read(&self, tile_part_bytes: u64, packet_body_bytes: u64) {
        self.inner
            .record_bytes_not_read(tile_part_bytes, packet_body_bytes);
    }
}

fn read_size_bucket(bytes: u64) -> usize {
    if bytes == 0 {
        0
    } else {
        1 + (u64::BITS - (bytes - 1).leading_zeros()) as usize
    }
}

fn read_size_bucket_upper_bound(bucket: usize) -> u64 {
    match bucket {
        0 => 0,
        1..=64 => 1_u64 << (bucket - 1),
        _ => u64::MAX,
    }
}

fn read_size_percentile_upper_bound(histogram: &[u64; 66], percentile: u64) -> u64 {
    let total = histogram
        .iter()
        .fold(0_u64, |total, count| total.saturating_add(*count));
    if total == 0 {
        return 0;
    }
    let rank = total.saturating_mul(percentile).div_ceil(100).max(1);
    let mut cumulative = 0_u64;
    for (bucket, count) in histogram.iter().copied().enumerate() {
        cumulative = cumulative.saturating_add(count);
        if cumulative >= rank {
            return read_size_bucket_upper_bound(bucket);
        }
    }
    u64::MAX
}

/// Atomic instrumentation wrapper safe for parallel positioned reads.
#[derive(Debug)]
pub struct InstrumentedSource<S> {
    inner: S,
    bytes_requested: AtomicU64,
    bytes_returned: AtomicU64,
    read_operations: AtomicU64,
    ranges_coalesced: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
    largest_read: AtomicU64,
    read_size_histogram: [AtomicU64; 66],
    tile_part_bytes_not_read: AtomicU64,
    packet_body_bytes_not_read: AtomicU64,
}

impl<S> InstrumentedSource<S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            bytes_requested: AtomicU64::new(0),
            bytes_returned: AtomicU64::new(0),
            read_operations: AtomicU64::new(0),
            ranges_coalesced: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            largest_read: AtomicU64::new(0),
            read_size_histogram: core::array::from_fn(|_| AtomicU64::new(0)),
            tile_part_bytes_not_read: AtomicU64::new(0),
            packet_body_bytes_not_read: AtomicU64::new(0),
        }
    }

    pub fn inner(&self) -> &S {
        &self.inner
    }

    pub fn metrics(&self) -> SourceMetrics {
        let read_size_histogram = self
            .read_size_histogram
            .each_ref()
            .map(|count| count.load(Ordering::Relaxed));
        SourceMetrics {
            source_bytes_requested: self.bytes_requested.load(Ordering::Relaxed),
            source_bytes_returned: self.bytes_returned.load(Ordering::Relaxed),
            source_read_operations: self.read_operations.load(Ordering::Relaxed),
            source_ranges_coalesced: self.ranges_coalesced.load(Ordering::Relaxed),
            source_cache_hits: self.cache_hits.load(Ordering::Relaxed),
            source_cache_misses: self.cache_misses.load(Ordering::Relaxed),
            largest_source_read: self.largest_read.load(Ordering::Relaxed),
            median_source_read_upper_bound: read_size_percentile_upper_bound(
                &read_size_histogram,
                50,
            ),
            p95_source_read_upper_bound: read_size_percentile_upper_bound(&read_size_histogram, 95),
            tile_part_bytes_not_read: self.tile_part_bytes_not_read.load(Ordering::Relaxed),
            packet_body_bytes_not_read: self.packet_body_bytes_not_read.load(Ordering::Relaxed),
        }
    }

    pub fn reset_metrics(&self) {
        for counter in [
            &self.bytes_requested,
            &self.bytes_returned,
            &self.read_operations,
            &self.ranges_coalesced,
            &self.cache_hits,
            &self.cache_misses,
            &self.largest_read,
            &self.tile_part_bytes_not_read,
            &self.packet_body_bytes_not_read,
        ] {
            counter.store(0, Ordering::Relaxed);
        }
        for count in &self.read_size_histogram {
            count.store(0, Ordering::Relaxed);
        }
    }
}

impl<S: CodestreamSource> CodestreamSource for InstrumentedSource<S> {
    fn len(&self) -> Result<u64, SourceError> {
        self.inner.len()
    }

    fn read_exact_at(&self, offset: u64, destination: &mut [u8]) -> Result<(), SourceError> {
        let requested = destination.len() as u64;
        self.bytes_requested.fetch_add(requested, Ordering::Relaxed);
        self.read_operations.fetch_add(1, Ordering::Relaxed);
        self.largest_read.fetch_max(requested, Ordering::Relaxed);
        self.read_size_histogram[read_size_bucket(requested)].fetch_add(1, Ordering::Relaxed);
        let result = self.inner.read_exact_at(offset, destination);
        if result.is_ok() {
            self.bytes_returned.fetch_add(requested, Ordering::Relaxed);
        }
        result
    }

    fn metrics(&self) -> Option<SourceMetrics> {
        Some(self.metrics())
    }

    fn record_coalesced_range(&self) {
        self.ranges_coalesced.fetch_add(1, Ordering::Relaxed);
    }

    fn record_bytes_not_read(&self, tile_part_bytes: u64, packet_body_bytes: u64) {
        self.tile_part_bytes_not_read
            .fetch_add(tile_part_bytes, Ordering::Relaxed);
        self.packet_body_bytes_not_read
            .fetch_add(packet_body_bytes, Ordering::Relaxed);
    }
}

#[cfg(feature = "std")]
mod std_sources {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use std::io::{Read, Seek, SeekFrom};
    use std::path::Path;
    use std::sync::Mutex;

    /// Portable cursor-independent file adapter. A mutex supplies positioned
    /// semantics on platforms without a shared `read_at` primitive.
    #[derive(Debug)]
    pub struct StdFileSource {
        file: Mutex<std::fs::File>,
        len: u64,
    }

    impl StdFileSource {
        pub fn open(path: impl AsRef<Path>) -> Result<Self, SourceError> {
            let file = std::fs::File::open(path.as_ref())
                .map_err(|error| SourceError::io(0, 0, 0, error))?;
            Self::new(file)
        }

        pub fn new(file: std::fs::File) -> Result<Self, SourceError> {
            let len = file
                .metadata()
                .map_err(|error| SourceError::io(0, 0, 0, error))?
                .len();
            Ok(Self {
                file: Mutex::new(file),
                len,
            })
        }
    }

    impl CodestreamSource for StdFileSource {
        fn len(&self) -> Result<u64, SourceError> {
            Ok(self.len)
        }

        fn read_exact_at(&self, offset: u64, destination: &mut [u8]) -> Result<(), SourceError> {
            let requested = destination.len() as u64;
            let available = self.len.saturating_sub(offset);
            if offset.saturating_add(requested) > self.len {
                return Err(SourceError::range(
                    SourceErrorKind::ShortRead,
                    offset,
                    requested,
                    available,
                ));
            }
            let mut file = self.file.lock().map_err(|_| SourceError {
                kind: SourceErrorKind::Io,
                offset,
                requested,
                available,
                message: String::from("file source lock is poisoned"),
            })?;
            file.seek(SeekFrom::Start(offset))
                .and_then(|_| file.read_exact(destination))
                .map_err(|error| SourceError::io(offset, requested, available, error))
        }
    }

    #[derive(Debug)]
    struct CacheWindow {
        offset: u64,
        bytes: Vec<u8>,
    }

    /// One-window bounded cache. Mutable cache state is an optimization only;
    /// logical source positions remain explicit and cursor-independent.
    #[derive(Debug)]
    pub struct StdCachingSource<S> {
        inner: S,
        window_bytes: usize,
        window: Mutex<Option<CacheWindow>>,
        hits: AtomicU64,
        misses: AtomicU64,
    }

    impl<S: CodestreamSource> StdCachingSource<S> {
        pub fn new(inner: S, window_bytes: usize) -> Result<Self, SourceError> {
            if window_bytes == 0 {
                return Err(SourceError::range(SourceErrorKind::OutOfRange, 0, 0, 0));
            }
            Ok(Self {
                inner,
                window_bytes,
                window: Mutex::new(None),
                hits: AtomicU64::new(0),
                misses: AtomicU64::new(0),
            })
        }

        pub fn inner(&self) -> &S {
            &self.inner
        }
    }

    impl<S: CodestreamSource> CodestreamSource for StdCachingSource<S> {
        fn len(&self) -> Result<u64, SourceError> {
            self.inner.len()
        }

        fn read_exact_at(&self, offset: u64, destination: &mut [u8]) -> Result<(), SourceError> {
            if destination.len() > self.window_bytes {
                self.misses.fetch_add(1, Ordering::Relaxed);
                return self.inner.read_exact_at(offset, destination);
            }
            let requested = destination.len() as u64;
            let source_len = self.inner.len()?;
            let available = source_len.saturating_sub(offset);
            let end = offset.checked_add(requested).ok_or_else(|| {
                SourceError::range(SourceErrorKind::OutOfRange, offset, requested, available)
            })?;
            if end > source_len {
                return Err(SourceError::range(
                    SourceErrorKind::ShortRead,
                    offset,
                    requested,
                    available,
                ));
            }
            let mut window = self.window.lock().map_err(|_| SourceError {
                kind: SourceErrorKind::Io,
                offset,
                requested,
                available: 0,
                message: String::from("source cache lock is poisoned"),
            })?;
            if let Some(cached) = window.as_ref() {
                let local = offset.checked_sub(cached.offset);
                if let Some(local) = local.and_then(|value| usize::try_from(value).ok())
                    && let Some(bytes) = local
                        .checked_add(destination.len())
                        .and_then(|end| cached.bytes.get(local..end))
                {
                    destination.copy_from_slice(bytes);
                    self.hits.fetch_add(1, Ordering::Relaxed);
                    return Ok(());
                }
            }

            self.misses.fetch_add(1, Ordering::Relaxed);
            let read_len =
                usize::try_from((self.window_bytes as u64).min(source_len.saturating_sub(offset)))
                    .map_err(|_| SourceError::range(SourceErrorKind::OutOfRange, offset, 0, 0))?;
            if read_len < destination.len() {
                return Err(SourceError::range(
                    SourceErrorKind::ShortRead,
                    offset,
                    destination.len() as u64,
                    read_len as u64,
                ));
            }
            let mut bytes = vec![0_u8; read_len];
            self.inner.read_exact_at(offset, &mut bytes)?;
            destination.copy_from_slice(&bytes[..destination.len()]);
            *window = Some(CacheWindow { offset, bytes });
            Ok(())
        }

        fn metrics(&self) -> Option<SourceMetrics> {
            let mut metrics = self.inner.metrics().unwrap_or_default();
            metrics.source_cache_hits = metrics
                .source_cache_hits
                .saturating_add(self.hits.load(Ordering::Relaxed));
            metrics.source_cache_misses = metrics
                .source_cache_misses
                .saturating_add(self.misses.load(Ordering::Relaxed));
            Some(metrics)
        }

        fn record_coalesced_range(&self) {
            self.inner.record_coalesced_range();
        }

        fn record_bytes_not_read(&self, tile_part_bytes: u64, packet_body_bytes: u64) {
            self.inner
                .record_bytes_not_read(tile_part_bytes, packet_body_bytes);
        }
    }
}

#[cfg(feature = "std")]
pub use std_sources::{StdCachingSource as CachingSource, StdFileSource as FileSource};
