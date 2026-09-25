# Classic forward 5/3 caller boundary v1 — source archive

The completed prospective experiment found **no worthwhile supported repair at
this budget**. The owning-helper boundary is present in C's ordinary executable,
but C/B's primary 99% interval contains zero: neither a benefit nor an adverse
intervention effect is resolved. Production is unchanged. The exact C source and
authored tests remain a source-only archive, with no selected repair, enabled
option, automatic next experiment or qualification campaign. The closed
[parallel-dispatch outcome](forward53-parallel-dispatch.md) and
[serial attribution](forward53-serial-attribution.md) remain unchanged.

## Prospective measured outcome

The full Boca RGB8/style-zero/one-worker endpoint completed all 126 starts:
three ordinary preflights, three separate allocation calls and 120 ordinary
observations (40 per arm). There were no failures, missing receipts, replacements
or unstarted slots. All invocations matched the 14,339,292-byte reference stream
and every reconstructed sample. The single installed balanced-reusable
reservation used physical CPU 0, its reserved unused SMT sibling and an off-core
controller; independent restoration passed. Acquisition consumed 653.847358
seconds of the 45-minute ceiling. Separate allocation calls each observed
362,401,748 additional requested peak bytes and 707 successful allocation or
reallocation requests, within the unchanged bounds; those calls supplied no
timing samples. Allocation requests, process RSS and CPU time remain distinct.

The ordinary means were A **2,480.778231 ms**, B **2,457.900420 ms**, and
C **2,460.321638 ms**. Positive time change means slower; positive saving favours
the numerator treatment.

| Contrast | Mean time change ms | Mean saving ms | Relative change | 99% relative-time interval | Legacy ±5% verdict |
|---|---:|---:|---:|---|---|
| C/B, sole primary | +2.421219 | −2.421219 | +0.098508% | [−0.120744%, +0.318239%] | Equivalent |
| B/A, present archived cost | −22.877811 | +22.877811 | −0.922203% | [−1.136246%, −0.707702%] | Equivalent |
| C/A, distance from reference | −20.456592 | +20.456592 | −0.824604% | [−1.039415%, −0.609333%] | Equivalent |

The unchanged ratio-of-arithmetic-means comparator used each arm's complete
40-observation vector once per contrast. The fixed three-arm schedule retained
20 occurrences of each relative pair ordering and position counts A 14/12/14,
B 13/14/13, C 13/14/13. Pairs were not always adjacent; this was the reviewed
three-arm extension, not the historical two-arm AB/BA protocol. Contrasts share
observations, so these are individual 99% intervals, not joint 99% coverage or
120 independent pairs. No trimming, pooling, additional rounds or alternative
estimator was used.

C/B's upper bound is above zero, so the frozen intervention-benefit screen fails.
C/A's upper bound is below +1%, satisfying the existing serial bound for this
endpoint only. That favourable C/A result cannot establish that the caller
boundary helped. The source simplification therefore has no supported measured
gain warranting selection as a repair at this budget. The small positive C/B
point estimate remains an unresolved adverse direction, not a proven regression.
B/A did not reproduce the historical slowdown; this prospective result does not
revise the closed historical confirmation or identify its cause. No full-matrix
qualification or production promotion follows.

The benchmark owns acquisition, all samples, allocation/resource/environment
receipts, comparator output and restoration evidence. Its final report SHA-256
is `25bd665805f4c15076b83c7b8030415a9f12b0920d2c396a99adf97b67cb92c8`.
The actual A/B/C executables, symbols, source inventories and build/linker
receipts remain in the approved evidence store. The compiled interpretation
below limits the narrower causal claim even though the experiment itself is
valid and complete.

## Exact source and declared intervention

| Identity | Value |
|---|---|
| A, production reference | `a7576ad03486e097ac923b8e49cac39a1cbef5d2` |
| B, archived dispatch | `4f5bfb39f02e043c9a1f594a8159d3cf86d52c3f` |
| B tree | `259066112bda675e67a8d3eab3dbf9088d18fe91` |
| C, boundary experiment | `c6c07420e90cd7b871f7e6b0e3bc813ab384bf8c` |
| C tree | `ff33aaeac39abbbf2d55f4cfa24f2afbfe58c0db` |
| B-to-C full-index patch | [classic-forward53-caller-boundary.patch.txt](performance-candidates/classic-forward53-caller-boundary.patch.txt) |
| Patch SHA-256 | `3224ce3fa2412f2317aac3bdb24abcfefe7baca5979eb5e93fb3ac3c358fc9b1` |

Recover B using its existing archive and then apply this patch in an isolated
index. The [inventory recovery procedure](performance-candidates.md#exact-source-and-recovery)
can prove the C tree without a build or corpus operation. The patch changes only
`crates/emuella-j2k-codestream/src/scalable_lossless.rs`; it includes the authored
tests. Neither the historical archive nor the production source is replaced.

The single declared intervention adds `transform_forward53_parallel`, with one
prospectively selected `#[inline(never)]`. It owns preparation, execution of every
component, and destruction of panel workspace. Its `Result<bool>` returns only
completion or a decline before mutation through the existing error convention.
The large optional plan/workspace stays inside this boundary; no boxing or new
allocation substitutes for its stack representation.

The writer tests effective admission outside the component loop. With one
worker it skips the owning helper and runs the original scalar multi-level
helper with its reusable `Vec` scratch. Potential parallel work enters the new
helper and retains B's width-16 policy, slot/memory eligibility, optional
reservation retries and panel engine. A declined preparation returns before any
coefficient changes; an error after execution starts propagates without an
operation restart. Existing scalar handling within mixed LL plans remains
inside the planned engine. Successful helper destruction finishes before
exponents, entropy, packet and output allocations. Error unwinding also owns and
drops the workspace locally.

The separate diagnostic build preserves its existing forced scalar-panel route,
including one-worker forcing and invalid-width rejection. Ordinary builds have
neither forcing nor the test-only entry/failure hooks. No lifting, horizontal
work, scatter, joins, LL ordering, conversion, RCT, exponents, Tier-1, MQ, W,
packet assembly, public API, dependency or unsafe code changes are introduced.

## Authored correctness

Five new tests exercise the moved boundary: an unchanged-plane pre-mutation
decline; complete three-component parity at two, four and eight workers with
mixed LL levels and reuse; a malformed second component that propagates error
after the first component has changed without restarting or touching the third;
optional preparation failure after reservation followed by safe scalar fallback;
and writer entry counts proving that effective single-worker admission skips
the helper even inside larger pools. Existing optional reservation tests retain
slot reduction, memory-ineligible plans, release and fallback checks.

The recovered C canonical gate also exercises the existing full authored stream
matrix for both styles, admitted U8/U16 component/layout models, ordinary routing
and scratch capacities, tight budgets, small/ineligible geometry, mixed LL
levels, output failure and native decoding. Unchanged transform tests include
joined panic, no scatter after failed gather and workspace reuse. These tests
supply correctness evidence only, with no performance conclusion.

The canonical committed-tree check passed C revision
`c6c07420e90cd7b871f7e6b0e3bc813ab384bf8c`, including native workspace,
parallel routing, authored stream/transform tests, Clippy, legal/dependency
checks and no-default-feature compilation. A separate isolated-target
`cargo check -p emuella-j2k --no-default-features --target wasm32-unknown-unknown`
also passed. Existing dead-code warnings in unsupported no-std decoder routes
remain warnings; they are not new boundary failures. The full-index recovery
check reproduced the C tree exactly. No corpus call is part of these checks.

## Actual ordinary compiled result

The [bounded extracts](evidence/classic-forward53-caller-boundary/README.md) and
[manifest](evidence/classic-forward53-caller-boundary/static.json) bind the actual
one-build-per-arm ordinary executables. The reused extractor validates every
selected symbol's complete decoded byte extent and retains exact linked
addresses and bytes alongside limited normalisation. Separate whole-file,
`.text`, read-only/relocated data, unwind and debug hashes remain in the manifest.
These rebuilt treatments do not establish historical binary identity.

| Identity | A | B | C |
|---|---|---|---|
| Whole ELF SHA-256 | `459734d9abb32f6090ccaeb6ba660fe26cc95b386a93248554917ffa86b29664` | `15c1a64ae941a64849a5c2b440bd1f28c760de4ff3af97d01f79e9b2fcff7019` | `f4167cafc6065ba11d0c2267476a362710bc8d8e2bb0475e44cfbb97e67e4a91` |
| `.text` SHA-256 | `93b89dfd851ab69a519671303142b2c01603fd734872f4124c553276018ac7aa` | `2549e52ad8da0279db27149cdfd021410a6f8290a1286080bb9db16efb615e5c` | `c3a251c849d3718cf0c75aa021ed05bbb8b82b4ff9e88b2a361121643b67fbd0` |
| `.text` bytes | 2,113,100 | 2,136,348 | 2,136,604 |
| Writer address / bytes | `0x150110` / 5,133 | `0x151990` / 5,543 | `0x151cc0` / 5,197 |
| Writer explicit stack reservation | `0x318` (792) | `0x478` (1,144) | `0x348` (840) |
| Component closure address / bytes | `0xcecd0` / 191 | `0xd0090` / 291 | `0xc5070` / 332 |
| Component closure explicit stack reservation | `0x38` (56) | `0x58` (88) | `0x38` (56) |
| Preparation address / bytes | Absent | `0x151540` / 1,102 | Inlined into owning helper |
| Owning helper address / bytes / explicit stack reservation | Absent | Absent | `0x9b390` / 2,040 / `0x468` (1,128) |

All three writers and component closures save six registers before their listed
stack reservation. The closure also adjusts the stack for original-helper call
arguments; listed reservations are the explicit prologue subtraction, not a
whole-call-chain stack high-water measurement. C's writer reservation is 304
bytes below B and still 48 bytes above A. Its closure is larger in instructions
than either reference, despite the smaller reservation than B.

In B, the writer calls preparation at `0x152026`. For one worker, preparation's
`cmp $2,%rcx` at `0x151561` leads to its absent-option return at `0x151586`.
The writer still executes two fixed 176-byte `memcpy` calls at `0x1520a7` and
`0x1520cf` before invoking the component closure at `0x15214d`. These optional
metadata copies are absent from C's writer. The later ordinary output/header
copy remains; this is not a claim that encoding performs no copies.

C's writer invokes its component closure at `0x1523f0`. In that closure,
`cmp $1,%rcx` at `0xc5087` and `jbe` at `0xc508b` send effective single-worker
admission directly to `0xc50f3`, bypassing the owning-helper call at `0xc50b1`.
The original multi-level helper is called at `0xc514b` within the component
loop. No plan/workspace value crosses this boundary. Eligible calls return
completion/decline or the existing error representation; `Result<bool>` does
not imply a one-byte machine ABI for its error cases.

The new helper contains the preparation code and its fixed optional-state
moves. It owns the component loop at `0x9b610`; its indirect call at `0x9b62b`
uses `%r14`, loaded at `0x9b5fb` from relocation slot `0x2752e0`. That slot
resolves to Emuella's planned transform at `0x20a410`. The retained extracts
show preparation calls to `Forward53Plan::new` and
`Forward53Workspace::prepare`. Normal completion frees workspace at `0x9b65f`
before writing true at `0x9b665`; errors free locally before returning or
resuming unwinding. The caller releases original scalar scratch at `0x15246a`
before subband-specification allocation at `0x152480`. Thus the requested
ownership/execution/destruction boundary is present in the actual ordinary C
executable, and the serial path skips it entirely.

| Selected Emuella boundary | Bytes A / B / C | Normalised comparison |
|---|---|---|
| Worker encode / facade | 2,025 / 2,025 / 2,025; 1,831 / 1,831 / 1,831 | Same across all arms |
| Original multi-level / bounded 2D | 981 / 981 / 981; 1,380 / 1,380 / 1,380 | Same across all arms |
| Low-first / high-first line | 1,430 / 1,438 / 1,438; 1,313 / 1,327 / 1,327 | B and C same; A differs |
| Serial subband / Tier-1 strided / prepared | 2,606 / 506 / 5,789 in each arm | Same across all arms |
| Cleanup / significance / magnitude | 4,887 / 1,387 / 1,240 in each arm | Same across all arms |
| MQ write-bit / byte-out | 230 / 297 in each arm | Same across all arms |
| Main / packet headers / output reservation | 2,539 / 4,952 / 319 in each arm | Same across all arms |

The B/C low-first helper retains its register, stack-offset and instruction
sequence under the documented normalisation, including the earlier odd-path
code-generation difference from A. Its raw bytes differ, and it moves from
`0x206540` to `0x206600`. B/C bounded 2D likewise moves from `0x205730` to
`0x2057f0`; both changes preserve those functions' address modulo 64. The writer
moves from modulo 64 equal to 16 in B to 0 in C. Overall `.text` grows by 256
bytes from B to C. Matching normalised helpers do not prove identical relocation
targets, placement or runtime effects. Inlined conversion/RCT and assembly
remain in the changed writer, so their register/stack/layout context is also
part of this refactoring. The effect belongs to the complete source-level
intervention; the narrower caller-state mechanism is only partially isolated.
No static copy, instruction or stack-byte count is converted to milliseconds.
