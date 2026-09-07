//! Experimental tile-bounded lossy HT encoding and reusable regional dependencies.
//!
//! This module has no transport, file or threading dependency. Its opaque index
//! belongs to the exact byte stream written by `encode_tiled`; callers own source
//! identity and lifetime. See `docs/ht-indexed-foundation.md` for the fixed profile.
use super::ht_lossy::{reserved, resource_error, zeroed};
use super::*;
use core::ops::Range;

/// Fixed two-level, one-layer, no-MCT, default-precinct HTONLY profile.
#[derive(Debug, Clone, Copy)]
pub struct TiledLossyHtProfile {
    pub width: u32,
    pub height: u32,
    /// Supported square tile edges: 256, 512 and 1024 samples.
    pub tile_edge: u32,
    pub bits_per_sample: u8,
    pub components: u16,
    /// Per-tile rate target, in bits per reference pixel, plus a 128-byte
    /// tile allowance. Finite search can undershoot; tight filling is not promised.
    pub bits_per_pixel: f32,
}

/// One actual packet of the generated single-layer codestream. There is one
/// default precinct per tile/component/resolution; the precinct ordinal is zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedHtPrecinct {
    pub tile: u16,
    pub component: u16,
    pub resolution: u8,
    pub precinct: u64,
    pub layer: u16,
    /// Absolute source byte ranges; concatenating these yields the packet.
    pub header: Range<u64>,
    pub body: Range<u64>,
}

struct IndexedTile {
    rect: TileRect,
    component: ComponentParameters,
    candidate: HtCodestreamDecodeCandidate,
    payload_offset: u64,
    contributions: Vec<PacketCodeBlockContribution>,
}

/// Encoder-built reusable metadata. It retains no source samples or encoded
/// payload. It is deliberately not a deserialisation or arbitrary-file parser.
pub struct IndexedLossyHt {
    profile: TiledLossyHtProfile,
    main_header: Range<u64>,
    tile_headers: Vec<Range<u64>>,
    precincts: Vec<IndexedHtPrecinct>,
    tiles: Vec<IndexedTile>,
    encoded_bytes: u64,
    contribution_heap_bytes: usize,
}

impl IndexedLossyHt {
    pub fn profile(&self) -> TiledLossyHtProfile {
        self.profile
    }
    /// Source main header, including SOC and ending before the first SOT.
    pub fn main_header(&self) -> Range<u64> {
        self.main_header.clone()
    }
    /// Tile header marker segments, excluding SOT and SOD delimiters.
    pub fn tile_headers(&self) -> &[Range<u64>] {
        &self.tile_headers
    }
    pub fn precincts(&self) -> &[IndexedHtPrecinct] {
        &self.precincts
    }
    pub fn encoded_bytes(&self) -> u64 {
        self.encoded_bytes
    }
    /// Retained dynamic metadata capacity, excluding allocator overhead.
    pub fn retained_heap_bytes(&self) -> u64 {
        let mut n = self.tiles.capacity() * core::mem::size_of::<IndexedTile>()
            + self.precincts.capacity() * core::mem::size_of::<IndexedHtPrecinct>()
            + self.tile_headers.capacity() * core::mem::size_of::<Range<u64>>();
        n += self.contribution_heap_bytes;
        n as u64
    }
}

fn validate_profile(p: TiledLossyHtProfile) -> Result<usize> {
    if p.width < 4
        || p.height < 4
        || !matches!(p.tile_edge, 256 | 512 | 1024)
        || (!p.width.is_multiple_of(p.tile_edge) && p.width % p.tile_edge < 4)
        || (!p.height.is_multiple_of(p.tile_edge) && p.height % p.tile_edge < 4)
        || !matches!(p.bits_per_sample, 8 | 16)
        || !matches!(p.components, 1 | 3)
        || !p.bits_per_pixel.is_finite()
        || p.bits_per_pixel <= 0.0
    {
        return Err(resource_error());
    }
    let count =
        u64::from(p.width.div_ceil(p.tile_edge)) * u64::from(p.height.div_ceil(p.tile_edge));
    if count > 65535 {
        return Err(resource_error());
    }
    usize::try_from(count).map_err(|_| resource_error())
}

/// Write one codestream from row-major tiles, retaining metadata once.
///
/// `read_tile` fills tightly packed planar unsigned bytes (U16 little-endian).
/// All plane lengths are fixed and checked after the callback. `write` consumes
/// chunks synchronously. A callback/encoding failure leaves a partial output;
/// callers must publish only after success. No full-image pixel or byte buffer
/// is allocated internally. The optional application sink may of course buffer.
pub fn encode_tiled(
    profile: TiledLossyHtProfile,
    mut read_tile: impl FnMut(TileRect, &mut [Vec<u8>]) -> Result<()>,
    mut write: impl FnMut(&[u8]) -> Result<()>,
) -> Result<IndexedLossyHt> {
    let count = validate_profile(profile)?;
    let mut index = IndexedLossyHt {
        profile,
        main_header: 0..0,
        tile_headers: reserved(count)?,
        precincts: reserved(count * usize::from(profile.components) * 3)?,
        tiles: reserved(count)?,
        encoded_bytes: 0,
        contribution_heap_bytes: 0,
    };
    let cols = profile.width.div_ceil(profile.tile_edge);
    for ordinal in 0..count {
        let tx = ordinal as u32 % cols;
        let ty = ordinal as u32 / cols;
        let x = tx * profile.tile_edge;
        let y = ty * profile.tile_edge;
        let rect = TileRect {
            tile_index: ordinal as u16,
            tile_x: tx,
            tile_y: ty,
            x,
            y,
            width: profile.tile_edge.min(profile.width - x),
            height: profile.tile_edge.min(profile.height - y),
        };
        let pixels = rect.width as usize * rect.height as usize;
        let plane_bytes = pixels * usize::from(profile.bits_per_sample / 8);
        let mut planes = reserved(usize::from(profile.components))?;
        for _ in 0..profile.components {
            planes.push(zeroed(plane_bytes, 0_u8)?);
        }
        read_tile(rect, &mut planes)?;
        if planes.iter().any(|plane| plane.len() != plane_bytes) {
            return Err(resource_error());
        }
        let mut analysed = reserved(planes.len())?;
        for plane in &planes {
            let mut values = reserved(pixels)?;
            for sample in plane.chunks_exact(usize::from(profile.bits_per_sample / 8)) {
                let v = if profile.bits_per_sample == 8 {
                    u16::from(sample[0])
                } else {
                    u16::from_le_bytes([sample[0], sample[1]])
                };
                values.push(f32::from(v) - (1_u32 << (profile.bits_per_sample - 1)) as f32);
            }
            analysed.push(values);
        }
        drop(planes);
        ht_lossy::analyse(rect.width, rect.height, &mut analysed)?;
        let raw_budget = ht_lossy::encode_byte_budget(
            rect.width,
            rect.height,
            profile.bits_per_sample,
            usize::from(profile.components),
            profile.bits_per_pixel,
        )?;
        // A fixed allowance admits narrow boundary tiles without assigning a
        // negative packet budget. Total overhead remains bounded by tile count.
        let budget = raw_budget.checked_add(128).ok_or_else(resource_error)?;
        let (tile, _, _) = ht_lossy::search_tile(
            rect.width,
            rect.height,
            profile.bits_per_sample,
            &analysed,
            budget,
        )?;
        if tile.len() > budget {
            return Err(resource_error());
        }
        drop(analysed);
        // Reuse canonical packet admission on one bounded local tile. This
        // temporary envelope is validation scratch, never an output chunk or
        // independent representation. Only the packet metadata survives.
        let mut local = tile.header.clone();
        local
            .try_reserve(tile.packets.len() + 16)
            .map_err(|_| resource_error())?;
        write_tile_part(&mut local, 0, &tile.packets, true)?;
        let parsed = parse(&local)?;
        let (local_rect, payload) = single_part1_profile_tile(&local, &parsed)?;
        let contributions =
            parse_default_precinct_lrcp_packets(&local, &parsed, local_rect, payload)?;
        validate_part15_single_ht_packet_sets(&parsed, &contributions)?;
        let mut contribution_bytes =
            contributions.capacity() * core::mem::size_of::<PacketCodeBlockContribution>();
        for c in &contributions {
            contribution_bytes += c.segment_ranges.capacity()
                * core::mem::size_of::<PacketCodeBlockSegmentRange>()
                + c.coding_segments.capacity() * core::mem::size_of::<tier1::CodeBlockSegment>();
            if let Some(sets) = &c.expanded_ht_coding_sets {
                contribution_bytes += sets.len() * core::mem::size_of::<HtCodeBlockCodingSet>();
            }
        }
        index.contribution_heap_bytes = index
            .contribution_heap_bytes
            .checked_add(contribution_bytes)
            .ok_or_else(resource_error)?;
        if index.retained_heap_bytes() > 256 * 1024 * 1024 {
            return Err(resource_error());
        }
        let component = parsed.siz.components[0];
        let candidate = ht_decode_candidate_with_transform_permission(&parsed, true)
            .and_then(core::result::Result::ok)
            .ok_or_else(resource_error)?;
        drop(local);
        if ordinal == 0 {
            let mut header = tile.header.clone();
            // Part 15:2019 A.3.5 / Table A.2 permits tile-dependent QCD
            // through Ccap^15 bit 11; SINGLEHT and other fields stay fixed.
            let cap = parsed
                .markers
                .iter()
                .find(|m| m.marker == Marker::Cap)
                .ok_or_else(resource_error)?;
            header[cap.data_offset + 4..cap.data_offset + 6]
                .copy_from_slice(&0x082a_u16.to_be_bytes());
            header[8..12].copy_from_slice(&profile.width.to_be_bytes());
            header[12..16].copy_from_slice(&profile.height.to_be_bytes());
            header[24..28].copy_from_slice(&profile.tile_edge.to_be_bytes());
            header[28..32].copy_from_slice(&profile.tile_edge.to_be_bytes());
            write(&header)?;
            index.encoded_bytes = header.len() as u64;
            index.main_header = 0..index.encoded_bytes;
        }
        let qcd = &tile.header[tile.header.len() - 19..];
        let part_len =
            u32::try_from(14 + qcd.len() + tile.packets.len()).map_err(|_| resource_error())?;
        let mut part_header = reserved(33)?;
        part_header.extend_from_slice(&[0xff, 0x90, 0, 10]);
        part_header.extend_from_slice(&(ordinal as u16).to_be_bytes());
        part_header.extend_from_slice(&part_len.to_be_bytes());
        part_header.extend_from_slice(&[0, 1]);
        let header_start = index.encoded_bytes + part_header.len() as u64;
        part_header.extend_from_slice(qcd);
        index
            .tile_headers
            .push(header_start..index.encoded_bytes + part_header.len() as u64);
        part_header.extend_from_slice(&[0xff, 0x93]);
        write(&part_header)?;
        let payload_offset = index.encoded_bytes + part_header.len() as u64;
        write(&tile.packets)?;
        for &(component, resolution, start, body, end) in &tile.packet_ranges {
            index.precincts.push(IndexedHtPrecinct {
                tile: ordinal as u16,
                component,
                resolution,
                precinct: 0,
                layer: 0,
                header: payload_offset + start as u64..payload_offset + body as u64,
                body: payload_offset + body as u64..payload_offset + end as u64,
            });
        }
        index.encoded_bytes += u64::from(part_len);
        index.tiles.push(IndexedTile {
            rect,
            component,
            candidate,
            payload_offset,
            contributions,
        });
    }
    write(&[0xff, 0xd9])?;
    index.encoded_bytes += 2;
    Ok(index)
}

struct WindowPart {
    tile: usize,
    component: u16,
    synthesis: SynthesisWindowPlan,
    selected: Vec<usize>,
    accounting: ht_lossy::LossyHtSpatialRegionAccounting,
    output: TileRegionRequest,
}

/// Reusable regional dependency plan borrowing immutable encoder metadata.
pub struct IndexedHtRegion<'a> {
    index: &'a IndexedLossyHt,
    output: TileRegionRequest,
    components: Vec<u16>,
    parts: Vec<WindowPart>,
    precinct_indices: Vec<usize>,
    block_ranges: Vec<Range<u64>>,
}

impl IndexedLossyHt {
    /// Plan contained full-grid half-open coordinates; discard 0, 1 or 2 levels.
    /// Only intersecting tiles' retained block metadata is visited. No source
    /// byte is read and no packet header is parsed. Output is bounded to 64 MiB.
    pub fn plan(
        &self,
        region: TileRegionRequest,
        discard: u8,
        components: &[u16],
    ) -> Result<IndexedHtRegion<'_>> {
        let p = self.profile;
        if discard > 2
            || region.width == 0
            || region.height == 0
            || components.is_empty()
            || region
                .x
                .checked_add(region.width)
                .is_none_or(|v| v > p.width)
            || region
                .y
                .checked_add(region.height)
                .is_none_or(|v| v > p.height)
            || components.iter().any(|&c| c >= p.components)
            || components
                .iter()
                .enumerate()
                .any(|(i, c)| components[..i].contains(c))
        {
            return Err(resource_error());
        }
        let scale = 1 << discard;
        let output = TileRegionRequest {
            x: region.x.div_ceil(scale),
            y: region.y.div_ceil(scale),
            width: (region.x + region.width).div_ceil(scale) - region.x.div_ceil(scale),
            height: (region.y + region.height).div_ceil(scale) - region.y.div_ceil(scale),
        };
        let bytes = u64::from(output.width)
            * u64::from(output.height)
            * components.len() as u64
            * u64::from(p.bits_per_sample / 8);
        if bytes == 0 || bytes > 64 * 1024 * 1024 {
            return Err(resource_error());
        }
        let mut plan = IndexedHtRegion {
            index: self,
            output,
            components: components.to_vec(),
            parts: Vec::new(),
            precinct_indices: Vec::new(),
            block_ranges: Vec::new(),
        };
        let cols = p.width.div_ceil(p.tile_edge);
        for ty in region.y / p.tile_edge..=(region.y + region.height - 1) / p.tile_edge {
            for tx in region.x / p.tile_edge..=(region.x + region.width - 1) / p.tile_edge {
                let ordinal = (ty * cols + tx) as usize;
                let tile = &self.tiles[ordinal];
                let x0 = region.x.max(tile.rect.x);
                let y0 = region.y.max(tile.rect.y);
                let x1 = (region.x + region.width).min(tile.rect.x + tile.rect.width);
                let y1 = (region.y + region.height).min(tile.rect.y + tile.rect.height);
                let local = AxisAlignedRegion {
                    x: (x0 - tile.rect.x).div_ceil(scale),
                    y: (y0 - tile.rect.y).div_ceil(scale),
                    width: (x1 - tile.rect.x).div_ceil(scale) - (x0 - tile.rect.x).div_ceil(scale),
                    height: (y1 - tile.rect.y).div_ceil(scale) - (y0 - tile.rect.y).div_ceil(scale),
                };
                if local.width == 0 || local.height == 0 {
                    continue;
                }
                for &component in components {
                    let synthesis = plan_synthesis_window(
                        tile.rect.width.div_ceil(scale),
                        tile.rect.height.div_ceil(scale),
                        2 - discard,
                        local,
                        WaveletTransform::Irreversible97,
                    )?;
                    let mut selected = Vec::new();
                    for (i, c) in tile.contributions.iter().enumerate() {
                        if c.component_index == component
                            && c.resolution <= 2 - discard
                            && synthesis_window_dependency_selects_contribution(&synthesis, c)?
                        {
                            selected.try_reserve(1).map_err(|_| resource_error())?;
                            selected.push(i);
                            plan.block_ranges.push(
                                tile.payload_offset + c.payload_offset as u64
                                    ..tile.payload_offset
                                        + (c.payload_offset + c.codeword_len) as u64,
                            );
                        }
                    }
                    for resolution in 0..=2 - discard {
                        // Empty packets also belong to dependency metadata.
                        plan.precinct_indices.push(
                            ordinal * usize::from(p.components) * 3
                                + usize::from(resolution) * usize::from(p.components)
                                + usize::from(component),
                        );
                    }
                    let accounting = ht_lossy::lossy_ht_window_storage_accounting(
                        &synthesis,
                        &tile.contributions,
                        &selected,
                    )?;
                    plan.parts.push(WindowPart {
                        tile: ordinal,
                        component,
                        synthesis,
                        selected,
                        accounting,
                        output: TileRegionRequest {
                            x: tile.rect.x / scale + local.x,
                            y: tile.rect.y / scale + local.y,
                            width: local.width,
                            height: local.height,
                        },
                    });
                }
            }
        }
        Ok(plan)
    }
}

impl IndexedHtRegion<'_> {
    pub fn output_region(&self) -> TileRegionRequest {
        self.output
    }
    pub fn precinct_indices(&self) -> &[usize] {
        &self.precinct_indices
    }
    /// Exact entropy bytes needed, distinct from complete precinct delivery.
    pub fn block_ranges(&self) -> &[Range<u64>] {
        &self.block_ranges
    }
    pub fn selected_code_blocks(&self) -> usize {
        self.block_ranges.len()
    }
    pub fn selected_block_coefficients(&self) -> u64 {
        self.parts
            .iter()
            .map(|p| p.accounting.selected_block_coefficients)
            .sum()
    }
    pub fn required_workspace_bytes(&self) -> u64 {
        self.parts
            .iter()
            .map(|p| p.accounting.deterministic_workspace_ceiling_bytes)
            .max()
            .unwrap_or(0)
    }
    /// Decode requested planes in selection order, reading selected entropy
    /// blocks only. A callback must fill every requested byte or return an
    /// error. Missing compressed data is never treated as a zero contribution.
    /// No output is published on failure. Reusing a workspace avoids repeated
    /// coefficient/synthesis allocations, but does not cache decoded blocks.
    pub fn decode(
        &self,
        mut read: impl FnMut(u64, &mut [u8]) -> Result<()>,
        workspace: &mut ht_lossy::LossyHtSpatialRegionWorkspace,
    ) -> Result<Vec<Vec<u8>>> {
        if self.required_workspace_bytes() > workspace.maximum_bytes() {
            return Err(resource_error());
        }
        let sample_bytes = usize::from(self.index.profile.bits_per_sample / 8);
        let stride = self.output.width as usize * sample_bytes;
        let mut planes = reserved(self.components.len())?;
        for _ in &self.components {
            planes.push(zeroed(stride * self.output.height as usize, 0_u8)?);
        }
        for part in &self.parts {
            let tile = &self.index.tiles[part.tile];
            let (samples, _) = ht_lossy::decode_indexed_window(
                tile.candidate,
                &tile.component,
                &tile.contributions,
                &part.synthesis,
                &part.selected,
                &part.accounting,
                workspace,
                |c, buffer| {
                    buffer
                        .try_reserve(c.codeword_len)
                        .map_err(|_| resource_error())?;
                    buffer.resize(c.codeword_len, 0);
                    read(tile.payload_offset + c.payload_offset as u64, buffer)
                },
            )?;
            let plane = &mut planes[self
                .components
                .iter()
                .position(|&c| c == part.component)
                .ok_or_else(resource_error)?];
            let row_bytes = part.output.width as usize * sample_bytes;
            for y in 0..part.output.height as usize {
                let dst = (part.output.y - self.output.y) as usize * stride
                    + y * stride
                    + (part.output.x - self.output.x) as usize * sample_bytes;
                plane[dst..dst + row_bytes]
                    .copy_from_slice(&samples[y * row_bytes..(y + 1) * row_bytes]);
            }
        }
        Ok(planes)
    }
}

#[cfg(test)]
mod tests;
