# Classic forward 5/3 serial attribution v1

`classic-forward53-serial-attribution/v1` closes as **bounded unresolved static
attribution**. The recovered candidate changes generated work on the ordinary
one-worker style-zero path, despite retaining the original scalar transform
source. The evidence does not identify the cause or magnitude of the historical
serial timing difference. No supported repair is selected. Production and the
[parallel-dispatch qualification outcome](forward53-parallel-dispatch.md) remain
unchanged.

The historical Boca RGB8 style-zero one-worker means were 2434.577962 ms and
2457.712319 ms: a 23.134357 ms increase, with the complete 40-pair 99% relative-time
interval [+0.769086%, +1.131692%]. This did not resolve the required +1% upper
bound and did not establish a regression greater than 1%. This investigation
makes no new timing claim: ordinary corpus calls, stage diagnostics, sampling,
allocation diagnostics and profiler runs were all unperformed, with **zero new
starts against the 24-start ceiling**. “Cost not reproduced” would be an
unsupported description because there are no current timings.

## Binding and method

| Item | Baseline | Candidate |
|---|---|---|
| Codec source revision | `a7576ad03486e097ac923b8e49cac39a1cbef5d2` | `4f5bfb39f02e043c9a1f594a8159d3cf86d52c3f` |
| Codec tree | `5e7dfa48280c6d1d0abfe376d69af243e879359f` | `259066112bda675e67a8d3eab3dbf9088d18fe91` |
| Historical whole ELF SHA-256 | `a58280ce1c3f4c69c05b5258108b82d5a6e7724db74a77173ac6ac9b39deb187` | `682f2e85811157619c14b7635c24b8d3bcc289666269bb59f408902c535616f9` |
| Reconstructed whole ELF SHA-256 | `1066f9d95250b32bdf0be4dcc31efcb5ba9951fb2d8b567bf25916507222b335` | `6a29754a59c8be33b4a0777f60cbf1a49479924a1bd331e132e15546ceda73ea` |
| Reconstructed `.text` SHA-256 | `c7d788fd8704f0fe46c131165a4d3db547ffc1b793325f7a7fa740bb18b3acc2` | `53ca681df44c6f218a2d6c5eb410e5185311493aada3b321d0b6038368969af3` |
| Reconstructed `.text` bytes | 2,113,388 | 2,136,348 |

Candidate source was independently recovered from the existing full-index
archive. Both reconstructions use benchmark worker source
`0db375523ead4ff5113d7e73a77a39b04cff6644`, the ordinary `perf` profile
(optimisation level 3, ThinLTO, one codegen unit, line tables), the same Rust
compiler and the same lock SHA-256
`5cb1a2d7d5d098c347cc0ccdca0e52ca0315bb3190830ebd45d4c51f616788e8`.
The historical acquisition runner was separately
`7c5e24c4e07221e2f401b8ee4374f28a969dd406`; it is not the worker-source pin.
There was one successful ordinary build per arm, with no rebuild sweep.

The historical executable files and their individual section hashes are
unavailable. Historical inherited shell environment and linker executable/version
are also incomplete. Whole-file hash mismatch therefore cannot be assigned
solely to debug paths or metadata, and the reconstructed executable sections
cannot be certified as the sections that produced the historical interval.
Section identities for executable code, read-only data, relocated read-only
data, exception/unwind data and debug information are recorded separately in
[static.json](evidence/classic-forward53-serial-attribution/static.json).
Section hashing does not inspect external implementation internals.

The [bounded extractor](evidence/classic-forward53-serial-attribution/extract.py)
selects named Emuella-owned functions, preserves their instruction addresses and
bytes, resolves relevant indirect own-code calls from ELF relocations, and
asserts complete contiguous coverage against their ELF symbol extents. It never
executes a worker. External call labels are references only; no external function
body is extracted. The archive contains no input samples, coefficients,
codestreams, external binaries, standards text or private filesystem paths.
See the [evidence guide](evidence/classic-forward53-serial-attribution/README.md)
for replay and the intentionally limited normalisation.

## Actual route and compiled boundaries

The benchmark's `classic_compare_worker::encode` selects
`encode_with_limits` for style zero. Its resolved indirect call is at baseline
`0x6d84e` / candidate `0x6ecee`. The facade calls the ordinary codestream
`encode_lossless_d2` at `0x18d80c` / `0x18f20c`; relocation targets resolve to
`0x150330` / `0x151a50`. Its `encode_lossless_d2_impl::<false, false>` body is
inlined into that writer. This is the non-profiled, non-bypass instantiation,
not a diagnostic or bypass substitute.

| Boundary (archive key) | Baseline bytes | Candidate bytes | Normalised instruction sequence |
|---|---:|---:|---|
| Worker encode / facade (`worker_encode`, `facade`) | 2,025 / 1,831 | 2,025 / 1,831 | Same / same |
| Ordinary writer (`writer`) | 5,133 | 5,543 | Different |
| Component transform closure (`components`) | 191 | 291 | Different |
| Optional preparation (`prepare`) | Absent | 1,102 | Added |
| Original multi-level helper (`levels`) | 981 | 981 | Same |
| Original bounded 2D helper (`bounded53`) | 1,380 | 1,380 | Same |
| Low-first / high-first line (`low_line`, `high_line`) | 1,430 / 1,313 | 1,438 / 1,327 | Different / different |
| Serial subband (`subband`) | 2,606 | 2,606 | Same |
| Tier-1 strided / prepared (`tier1_strided`, `tier1_prepared`) | 506 / 5,789 | 506 / 5,789 | Same / same |
| Cleanup / significance / magnitude passes | 4,887 / 1,387 / 1,240 | 4,887 / 1,387 / 1,240 | Same |
| MQ write-bit / byte-out | 230 / 297 | 230 / 297 | Same / same |
| Main / packet headers | 2,539 / 4,952 | 2,539 / 4,952 | Same / same |
| Output reservation (`reserve_output`) | 319 | 319 | Same |

“Same” means only the documented normalised sequence matches; addresses,
relocations, referenced data and runtime behaviour are not proved equal.
Original addresses, bytes, hashes, call sites and static counts remain available
in the per-function extracts and manifest.

Input conversion and bounded RCT are inlined into the ordinary writer in both
arms. The U8 conversion remains a scalar byte-load, level-shift and `Vec` append
loop with bounds/capacity branches. Representative byte loads are baseline
`0x1506ea` / candidate `0x151dfd`; stack offsets and register allocation differ.
RCT retains the four-i32 SSE loop, including `paddd`, `psubd` and `psrad`, with
scalar remainder. Representative packed shifts are `0x1508dc` / `0x151ffc`.
There is no observed removal of RCT vectorisation.

Candidate `prepare_forward53` is an actual out-of-line call at `0x1520e6`.
For one admitted worker its `cmp $2,%rcx` / early-return branch reaches
`0x151646`, writes the absent-option discriminator and returns. It does not
construct a panel plan, reserve or initialise panel storage. Nevertheless, the
caller moves the fixed optional-result representation: two 176-byte stack copies
through the loaded `memcpy` pointer at `0x152167` and `0x15218f` are reachable
on this absent-option path. These are fixed metadata moves, not image-sized
copies or panel allocation. The component closure checks the discriminator at
`0xd01fd` for each plane and calls the original multi-level helper at `0xd01e3`.
The planned helper target at `0xd0216` is skipped. Successful cleanup checks the
optional storage discriminator at `0x152287` and skips its free for absence;
original scalar scratch is released before exponent scans and packet work.

The writer's explicit stack reservation increases from `0x318` to `0x478`
(792 to 1,144 bytes; six saved-register pushes in both). The component closure
reservation rises from `0x38` to `0x58`. Preparation reserves `0x2a8` even on its
early return. Static instructions with an explicit `(%rsp)` operand rise from
373 to 404 in the writer and 8 to 16 in the closure. These counts include address
formation, locals, result storage, error paths and argument handling. They are
neither measured memory accesses nor a spill count, and they do not estimate
executed instructions or cycles.

The original levels helper still resizes one reusable scalar scratch vector and
calls the bounded 2D helper twice per component. The latter retains column gather,
low/high-first line calls, strided store and horizontal line/copy boundaries.
Its low-first calls are baseline `0x2016c2` / `0x20180f` and candidate
`0x205af2` / `0x205c3f`. The source bodies of these original lifting helpers did
not change, but the low-first machine code did. For Boca's 5577 × 5036 then
2789 × 2518 active geometries, both vertical lengths are even and both horizontal
lengths are odd. Both axes are low-first; the high-first helper is not evidence
of executed work for this endpoint.

The even low-first prediction/update loops retain four-lane SSE arithmetic and
scalar tails. The prediction loop moves from `0x202270` to `0x2066a0` and its
index register changes; the update loop moves from `0x202400` to `0x206830`.
The odd path remains scalar and changes register allocation, bounds-check
scheduling and error exits. Baseline saves a value at `0x202512` for a later
error path; candidate saves at `0x206939` and reloads it at `0x20694f` on the
successful odd path before calculating a bound. This is a concrete static
spill/reload observation, not measured spill pressure. The subsequent odd
update loop changes bound registers and target layout. No added load per
sample or vectorisation loss is established by this one extra per-line reload.

After transformation, the writer still takes the one-worker serial subband
branch. The subband routine calls the strided Tier-1 entry, then prepared block
coding, cleanup/significance/magnitude passes and the project-authored MQ coder.
The extracted style-zero pass instantiations retain their normalised instruction
sequences. Packet-header construction stays out of line; output reservation,
resize, body movement and header copy remain in the writer's assembly phase.
Their writer register/stack placement changes, while the separately extracted
header and reservation routines retain normalised sequences. Neither this
finding nor unchanged source excludes instruction-layout or data/cache effects.

Layout is independently different: `.text` grows by 22,960 bytes, the writer's
address modulo 64 changes 48 → 16, the bounded 2D helper 0 → 48 and the low-first
helper 16 → 0. The two even lifting loop starts change modulo 64 from 48 → 32
and 0 → 48. These are static linked virtual-address observations, not cache-miss,
branch-prediction or instruction-fetch measurements.

## Competing explanations and stopping decision

| Hypothesis | Concrete evidence | Alternative / unresolved dynamic question |
|---|---|---|
| Serial caller state has a cost | Added preparation call, two fixed 176-byte copies, option checks and larger frames | The work is bounded per encode/component; no evidence gives its share of 23.134357 ms. Would a controlled caller-boundary isolation change total time while preserving the transform body? |
| Low-first code generation contributes | Changed odd-path register/bounds code and one successful-path reload; this endpoint has odd horizontal lines | Unchanged arithmetic and retained even-loop SIMD; code placement or unrelated runtime variation could dominate. Does causal isolation preserve or change this helper's bytes and measured share? |
| Conversion or assembly placement contributes | Their inlined writer registers/stack offsets change; RCT SIMD remains | No dynamic stage timings; source algorithm equality does not determine execution cost. |
| Layout contributes | Different `.text` extent, function locations and loop alignment | No sampled PCs, counters or controlled layout comparison; static addresses alone do not establish frontend cost. |
| Tier-1 algorithm changed | No supporting source or normalised instruction change in selected serial passes/MQ | Relocation/data/layout differences and runtime effects remain possible; normalisation cannot rule them out. |

An additional stage-timing or sampling setting could localise work in a new
reconstruction, but would not recover the historical executable or isolate a
cause. Instrumented builds could also change precisely the generated code under
investigation. The available static evidence therefore does not justify spending
new corpus calls merely to obtain another stage breakdown, and it does not
support choosing a production repair.

The one future decision is whether to commission a separately bounded causal
isolation of the serial caller boundary, with explicit source/build identities,
a finite measurement protocol and a check of resulting helper code. That is an
optional new investigation, not an authorised repair, retest, qualification or
promotion. This attribution package stops with the competing explanations above.
