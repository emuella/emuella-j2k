//! Fixed-size batches: exclusive reusable slots, shared planes, ordered append.
use super::*;
use rayon::prelude::*;

#[derive(Default)]
struct Slot {
    scratch: tier1::CodeBlockEncodeScratch,
    bytes: Vec<u8>,
    lengths: Vec<usize>,
    result: Option<Result<EncodedCodeBlock>>,
    worker: Option<usize>,
}

pub(super) struct BlockWorkers {
    slots: Vec<Slot>,
    // Allocated only by profiling; at most one entry per block, covered by B.
    participants: Vec<usize>,
}

impl BlockWorkers {
    pub(super) fn new(workers: usize) -> Result<Self> {
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(workers)
            .map_err(|_| CodestreamError::SizeOverflow)?;
        if slots.capacity() != workers {
            return Err(CodestreamError::SizeOverflow);
        }
        slots.resize_with(workers, Slot::default);
        Ok(Self {
            slots,
            participants: Vec::new(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn encode_subband<const PROFILE: bool, const BYPASS: bool>(
        &mut self,
        image_width: u32,
        plane: &[i32],
        spec: DecompSubbandSpec,
        available_bitplanes: u8,
        output: &mut Vec<u8>,
        maximum: usize,
        timings: &mut LosslessEncodeTimings,
        execution: &mut LosslessEncodeExecution,
    ) -> Result<(NativeDecompSubband, Vec<bypass::SegmentLengths>)> {
        // Only the validated scalable writer calls this path: origin-aligned
        // D2 subbands, with geometry admitted before coefficient allocation.
        if spec.grid_x0 != 0 || spec.grid_y0 != 0 || self.slots.is_empty() {
            return Err(CodestreamError::SizeOverflow);
        }
        let start = EncodeClock::start::<PROFILE>();
        let cols =
            u16::try_from(spec.width.div_ceil(64)).map_err(|_| CodestreamError::SizeOverflow)?;
        let rows =
            u16::try_from(spec.height.div_ceil(64)).map_err(|_| CodestreamError::SizeOverflow)?;
        let count = usize::from(cols)
            .checked_mul(usize::from(rows))
            .ok_or(CodestreamError::SizeOverflow)?;
        let mut blocks = Vec::new();
        blocks
            .try_reserve_exact(count)
            .map_err(|_| CodestreamError::SizeOverflow)?;
        if blocks.capacity() != count {
            return Err(CodestreamError::SizeOverflow);
        }
        let mut lengths = bypass::metadata(if BYPASS { count } else { 0 })?;
        if PROFILE {
            timings.block_preparation_ns += start.ns();
        }
        for first in (0..count).step_by(self.slots.len()) {
            let batch_len = self.slots.len().min(count - first);
            let active = &mut self.slots[..batch_len];
            let start = EncodeClock::start::<PROFILE>();
            // for_each joins *every* job, including when another job failed.
            // Results never escape this batch; error selection follows stream order.
            active
                .par_iter_mut()
                .enumerate()
                .for_each(|(offset, slot)| {
                    slot.bytes.clear();
                    if PROFILE {
                        slot.worker = rayon::current_thread_index();
                    }
                    slot.result = Some(encode_block::<BYPASS>(
                        image_width,
                        plane,
                        spec,
                        available_bitplanes,
                        cols,
                        first + offset,
                        &mut slot.bytes,
                        &mut slot.lengths,
                        &mut slot.scratch,
                    ));
                });
            if PROFILE {
                timings.tier1_ns += start.ns();
                execution.max_batch_blocks = execution.max_batch_blocks.max(batch_len);
            }
            let start = EncodeClock::start::<PROFILE>();
            for slot in active {
                let mut block = slot.result.take().ok_or(CodestreamError::SizeOverflow)??;
                if PROFILE {
                    if let Some(worker) = slot.worker
                        && !self.participants.contains(&worker)
                    {
                        self.participants.push(worker);
                    }
                    execution.participating_workers = self.participants.len().max(1);
                    timings.checked_tier1_blocks += 1;
                    timings.included_tier1_blocks += u64::from(block.included);
                    timings.tier1_coefficients += u64::from(block.width) * u64::from(block.height);
                    timings.tier1_coding_passes += u64::from(block.coding_passes);
                    timings.tier1_codeword_bytes += block.segment_len as u64;
                }
                let segments = if BYPASS {
                    if block.segment_len != slot.bytes.len() {
                        return Err(CodestreamError::SizeOverflow);
                    }
                    Some(bypass::SegmentLengths::checked(
                        block.coding_passes,
                        &slot.bytes,
                        &slot.lengths,
                    )?)
                } else {
                    None
                };
                reserve_output(output, slot.bytes.len(), maximum)?;
                block.segment_offset = output.len();
                output.extend_from_slice(&slot.bytes);
                blocks.push(block);
                if let Some(segments) = segments {
                    lengths.push(segments);
                }
            }
            if PROFILE {
                timings.assembly_ns += start.ns();
            }
        }
        Ok((
            NativeDecompSubband {
                index: spec.index,
                resolution: spec.resolution,
                kind: spec.kind,
                x: spec.x,
                y: spec.y,
                width: spec.width,
                height: spec.height,
                code_block_cols: cols,
                code_block_rows: rows,
                available_bitplanes,
                code_blocks: blocks,
            },
            lengths,
        ))
    }
}

#[allow(clippy::too_many_arguments)]
fn encode_block<const BYPASS: bool>(
    image_width: u32,
    plane: &[i32],
    spec: DecompSubbandSpec,
    available_bitplanes: u8,
    cols: u16,
    index: usize,
    bytes: &mut Vec<u8>,
    lengths: &mut Vec<usize>,
    scratch: &mut tier1::CodeBlockEncodeScratch,
) -> Result<EncodedCodeBlock> {
    let x = u16::try_from(index % usize::from(cols)).map_err(|_| CodestreamError::SizeOverflow)?;
    let y = u16::try_from(index / usize::from(cols)).map_err(|_| CodestreamError::SizeOverflow)?;
    let local_x = u32::from(x) * 64;
    let local_y = u32::from(y) * 64;
    let width =
        u16::try_from((spec.width - local_x).min(64)).map_err(|_| CodestreamError::SizeOverflow)?;
    let height = u16::try_from((spec.height - local_y).min(64))
        .map_err(|_| CodestreamError::SizeOverflow)?;
    let dimensions = tier1::CodeBlockDimensions::new(width, height).map_err(map_tier1_error)?;
    let offset = (spec.y as usize + local_y as usize)
        .checked_mul(image_width as usize)
        .and_then(|n| n.checked_add(spec.x as usize + local_x as usize))
        .ok_or(CodestreamError::SizeOverflow)?;
    let source = plane.get(offset..).ok_or(CodestreamError::SizeOverflow)?;
    let block_spec = tier1::CodeBlockEncodeSpec {
        dimensions,
        subband: spec.kind.tier1_subband(),
        available_bitplanes,
        code_block_style: u8::from(BYPASS),
    };
    let encoded = if BYPASS {
        tier1::encode_baseline_code_block_segments_with_strided_scratch(
            source,
            image_width as usize,
            block_spec,
            bytes,
            lengths,
            scratch,
        )
    } else {
        tier1::encode_baseline_code_block_with_strided_scratch(
            source,
            image_width as usize,
            block_spec,
            bytes,
            scratch,
        )
    }
    .map_err(map_tier1_error)?;
    Ok(EncodedCodeBlock {
        x,
        y,
        width,
        height,
        included: encoded.included,
        missing_bitplanes: encoded
            .missing_bitplanes
            .checked_add(1)
            .ok_or(CodestreamError::SizeOverflow)?,
        coding_passes: encoded.pass_count,
        segment_offset: 0,
        segment_len: encoded.byte_len,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joined_batch_retains_all_results_after_an_in_flight_error() {
        joined_batch_recovers::<false>();
        joined_batch_recovers::<true>();
    }

    fn joined_batch_recovers<const BYPASS: bool>() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap()
            .install(|| {
                let spec = DecompSubbandSpec {
                    index: 0,
                    resolution: 0,
                    kind: PacketSubbandKind::LowLow,
                    x: 0,
                    y: 0,
                    grid_x0: 0,
                    grid_y0: 0,
                    width: 256,
                    height: 64,
                };
                let mut plane = vec![0; 256 * 64];
                // Only the first block exceeds the declared bitplane bound. The
                // other three jobs must still finish before its error is observed.
                plane[0] = i32::MAX;
                let mut workers = BlockWorkers::new(4).unwrap();
                let mut output = vec![0x71; 13];
                assert!(
                    workers
                        .encode_subband::<false, BYPASS>(
                            256,
                            &plane,
                            spec,
                            1,
                            &mut output,
                            1 << 20,
                            &mut LosslessEncodeTimings::default(),
                            &mut LosslessEncodeExecution::default(),
                        )
                        .is_err()
                );
                assert_eq!(output, vec![0x71; 13]);
                assert!(workers.slots[0].result.is_none());
                for slot in &workers.slots[1..] {
                    assert!(slot.result.as_ref().unwrap().is_ok());
                }
                // Reuse after failure clears stale results and codewords.
                plane[0] = 0;
                let recovered = workers
                    .encode_subband::<false, BYPASS>(
                        256,
                        &plane,
                        spec,
                        1,
                        &mut output,
                        1 << 20,
                        &mut LosslessEncodeTimings::default(),
                        &mut LosslessEncodeExecution::default(),
                    )
                    .unwrap();
                assert!(recovered.0.code_blocks.iter().all(|b| !b.included));
            });
    }
}
