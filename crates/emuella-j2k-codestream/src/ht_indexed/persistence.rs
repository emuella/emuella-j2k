//! Project-authored, versioned metadata envelope; encoded image bytes stay external.
use super::*;
const MAGIC: &[u8; 8] = b"EHTIDX01";
const MAX_DESCRIPTOR: usize = 1024 * 1024;
const MAX_TILE_PAYLOAD: usize = 16 * 1024 * 1024;

pub(super) fn encode_descriptor(
    tile: u16,
    offset: u64,
    encoded: &ht_lossy::EncodedLossyTile,
) -> Result<Vec<u8>> {
    let mut out = reserved(256)?;
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&tile.to_le_bytes());
    out.extend_from_slice(&offset.to_le_bytes());
    out.extend_from_slice(&(encoded.header.len() as u32).to_le_bytes());
    out.extend_from_slice(&encoded.header);
    for &(_, _, start, body, end) in &encoded.packet_ranges {
        out.extend_from_slice(&((body - start) as u32).to_le_bytes());
        out.extend_from_slice(&((end - body) as u32).to_le_bytes());
        out.extend_from_slice(&encoded.packets[start..body]);
    }
    if out.len() > MAX_DESCRIPTOR || encoded.packets.len() > MAX_TILE_PAYLOAD {
        return Err(resource_error());
    }
    Ok(out)
}
struct Reader<'a>(&'a [u8]);
impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let out = self.0.get(..n).ok_or_else(resource_error)?;
        self.0 = &self.0[n..];
        Ok(out)
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(
            self.take(4)?.try_into().map_err(|_| resource_error())?,
        ))
    }
}
impl IndexedLossyHt {
    /// Create an empty, sparse index bound by the caller to one immutable source.
    /// The trusted application manifest supplies the profile, source length and
    /// main-header range. Import only the descriptors needed by current demands.
    pub fn sparse(
        profile: TiledLossyHtProfile,
        encoded_bytes: u64,
        main_header: Range<u64>,
    ) -> Result<Self> {
        validate_profile(profile)?;
        if main_header.start != 0
            || main_header.end == 0
            || main_header.end > 256
            || main_header.end >= encoded_bytes
        {
            return Err(resource_error());
        }
        Ok(Self {
            profile,
            main_header,
            tile_headers: Vec::new(),
            precincts: Vec::new(),
            tiles: Vec::new(),
            encoded_bytes,
            contribution_heap_bytes: 0,
            peak_tile_index_bytes: 0,
        })
    }
    /// Durable bounded metadata for one tile; contains headers and source ranges,
    /// never entropy bodies or pixels. Bind it to the same immutable source as
    /// the main manifest. This envelope is an application index, not JPIP syntax.
    pub fn export_tile_descriptor(&self, tile: u16) -> Result<&[u8]> {
        let i = self
            .tiles
            .binary_search_by_key(&tile, |t| t.rect.tile_index)
            .map_err(|_| resource_error())?;
        Ok(&self.tiles[i].descriptor)
    }
    /// Validate and atomically insert one selected tile. Duplicate tiles, unknown
    /// versions, malformed metadata and out-of-source ranges fail. No source read
    /// or full-image allocation occurs; temporary admission uses at most one tile.
    pub fn import_tile_descriptor(&mut self, bytes: &[u8]) -> Result<()> {
        if bytes.len() > MAX_DESCRIPTOR {
            return Err(resource_error());
        }
        let mut reader = Reader(bytes);
        if reader.take(8)? != MAGIC {
            return Err(resource_error());
        }
        let ordinal = u16::from_le_bytes(reader.take(2)?.try_into().map_err(|_| resource_error())?);
        let position = self
            .tiles
            .binary_search_by_key(&ordinal, |t| t.rect.tile_index)
            .err()
            .ok_or_else(resource_error)?;
        let payload_offset =
            u64::from_le_bytes(reader.take(8)?.try_into().map_err(|_| resource_error())?);
        let p = self.profile;
        if usize::from(ordinal) >= validate_profile(p)? {
            return Err(resource_error());
        }
        let cols = p.width.div_ceil(p.tile_edge);
        let tx = u32::from(ordinal) % cols;
        let ty = u32::from(ordinal) / cols;
        let rect = TileRect {
            tile_index: ordinal,
            tile_x: tx,
            tile_y: ty,
            x: tx * p.tile_edge,
            y: ty * p.tile_edge,
            width: p.tile_edge.min(p.width - tx * p.tile_edge),
            height: p.tile_edge.min(p.height - ty * p.tile_edge),
        };
        let header_len = reader.u32()? as usize;
        if header_len > 256 {
            return Err(resource_error());
        }
        let header = reader.take(header_len)?;
        let qcd_len = 7 + 6 * usize::from(p.decomposition_levels);
        let qcd = header
            .get(header_len.checked_sub(qcd_len).ok_or_else(resource_error)?..)
            .ok_or_else(resource_error)?;
        if qcd[..2] != [0xff, 0x5c] || qcd[4] != 0x62 {
            return Err(resource_error());
        }
        let base = u16::from_be_bytes([qcd[5], qcd[6]]);
        let exponent = (base >> 11) as u8;
        let mantissa = base & 0x7ff;
        let mut steps = Vec::new();
        for i in 0..1 + 3 * usize::from(p.decomposition_levels) {
            let gain = if i == 0 {
                0
            } else if i % 3 == 0 {
                2
            } else {
                1
            };
            let e = exponent.checked_add(gain).ok_or_else(resource_error)?;
            if e > 28 {
                return Err(resource_error());
            }
            steps.push(
                transform::IrreversibleQuantizationStep::new(e, mantissa)
                    .map_err(|_| resource_error())?,
            );
        }
        let mut canonical = Vec::new();
        write_irreversible_main_header(
            &mut canonical,
            rect.width,
            rect.height,
            p.bits_per_sample,
            p.components,
            false,
            p.decomposition_levels,
            &steps,
            true,
        )?;
        if canonical != header || self.main_header.end != header_len as u64 {
            return Err(resource_error());
        }
        let header_end = payload_offset.checked_sub(2).ok_or_else(resource_error)?;
        let header_start = header_end
            .checked_sub(qcd_len as u64)
            .ok_or_else(resource_error)?;
        if header_start < self.main_header.end + 12 {
            return Err(resource_error());
        }
        let mut payload = Vec::new();
        let mut precincts = Vec::new();
        for resolution in 0..=p.decomposition_levels {
            for component in 0..p.components {
                let header_bytes = reader.u32()? as usize;
                let body_bytes = reader.u32()? as usize;
                let start = payload.len();
                let body = start.checked_add(header_bytes).ok_or_else(resource_error)?;
                let end = body.checked_add(body_bytes).ok_or_else(resource_error)?;
                if header_bytes == 0 || end > MAX_TILE_PAYLOAD {
                    return Err(resource_error());
                }
                let source_end = payload_offset
                    .checked_add(end as u64)
                    .ok_or_else(resource_error)?;
                if source_end > self.encoded_bytes - 2 {
                    return Err(resource_error());
                }
                payload
                    .try_reserve(end - start)
                    .map_err(|_| resource_error())?;
                payload.extend_from_slice(reader.take(header_bytes)?);
                payload.resize(end, 0);
                precincts.push(IndexedHtPrecinct {
                    tile: ordinal,
                    component,
                    resolution,
                    precinct: 0,
                    layer: 0,
                    header: payload_offset + start as u64..payload_offset + body as u64,
                    body: payload_offset + body as u64..source_end,
                });
            }
        }
        if !reader.0.is_empty() {
            return Err(resource_error());
        }
        let source_end = payload_offset + payload.len() as u64;
        // Tile parts in this representation are contiguous and row-major.
        // Check neighbours in the sparse set without visiting unrelated blocks.
        if ordinal == 0 && header_start - 12 != self.main_header.end {
            return Err(resource_error());
        }
        if usize::from(ordinal) + 1 == validate_profile(p)? && source_end != self.encoded_bytes - 2
        {
            return Err(resource_error());
        }
        for i in position.saturating_sub(1)..(position + 1).min(self.tiles.len()) {
            let other = &self.tiles[i];
            let other_start = self.tile_headers[i].start - 12;
            let other_end = other.payload_offset
                + u64::try_from(
                    other
                        .candidate
                        .tile_part
                        .payload_len
                        .ok_or_else(resource_error)?,
                )
                .map_err(|_| resource_error())?;
            if other.rect.tile_index < ordinal {
                if other_end > header_start - 12
                    || (other.rect.tile_index + 1 == ordinal && other_end != header_start - 12)
                {
                    return Err(resource_error());
                }
            } else if source_end > other_start
                || (ordinal + 1 == other.rect.tile_index && source_end != other_start)
            {
                return Err(resource_error());
            }
        }
        write_tile_part(&mut canonical, 0, &payload, true)?;
        let parsed = parse(&canonical)?;
        let (local_rect, local_payload) = single_part1_profile_tile(&canonical, &parsed)?;
        let contributions =
            parse_default_precinct_lrcp_packets(&canonical, &parsed, local_rect, local_payload)?;
        validate_part15_single_ht_packet_sets(&parsed, &contributions)?;
        // Packet admission derives all geometry, bitplanes and quantisers from
        // canonical headers. Verify descriptor body boundaries against its result.
        for precinct in &precincts {
            let mut cursor = precinct.body.start;
            for c in contributions.iter().filter(|c| {
                c.component_index == precinct.component && c.resolution == precinct.resolution
            }) {
                if c.coding_passes != 1
                    || !c.ht_coded
                    || c.ht_coding_set_count() != 1
                    || c.codeword_len < 2
                    || c.available_bitplanes > 30
                    || c.missing_most_significant_bitplanes >= c.available_bitplanes
                    || payload_offset + c.payload_offset as u64 != cursor
                {
                    return Err(resource_error());
                }
                cursor = cursor
                    .checked_add(c.codeword_len as u64)
                    .ok_or_else(resource_error)?;
            }
            if cursor != precinct.body.end {
                return Err(resource_error());
            }
        }
        let candidate = ht_decode_candidate_with_transform_permission(&parsed, true)
            .and_then(core::result::Result::ok)
            .ok_or_else(resource_error)?;
        let component = parsed.siz.components[0];
        let contribution_bytes = contributions.capacity()
            * core::mem::size_of::<PacketCodeBlockContribution>()
            + contributions
                .iter()
                .map(|c| {
                    c.segment_ranges.capacity()
                        * core::mem::size_of::<PacketCodeBlockSegmentRange>()
                        + c.coding_segments.capacity()
                            * core::mem::size_of::<tier1::CodeBlockSegment>()
                })
                .sum::<usize>();
        let added = contribution_bytes
            + bytes.len()
            + core::mem::size_of::<IndexedTile>()
            + core::mem::size_of::<Range<u64>>()
            + precincts.len() * core::mem::size_of::<IndexedHtPrecinct>();
        if self.retained_heap_bytes() + added as u64 > 256 * 1024 * 1024 {
            return Err(resource_error());
        }
        self.tiles
            .try_reserve_exact(1)
            .map_err(|_| resource_error())?;
        self.tile_headers
            .try_reserve_exact(1)
            .map_err(|_| resource_error())?;
        self.precincts
            .try_reserve_exact(precincts.len())
            .map_err(|_| resource_error())?;
        let mut descriptor = reserved(bytes.len())?;
        descriptor.extend_from_slice(bytes);
        self.tiles.insert(
            position,
            IndexedTile {
                rect,
                component,
                candidate,
                payload_offset,
                contributions,
                descriptor,
            },
        );
        self.tile_headers.insert(position, header_start..header_end);
        self.precincts.extend(precincts);
        self.precincts
            .sort_unstable_by_key(|p| (p.tile, p.resolution, p.component));
        self.contribution_heap_bytes += contribution_bytes;
        Ok(())
    }
}
