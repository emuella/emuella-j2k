# Static evidence replay

These extracts accompany [the attribution finding](../../forward53-serial-attribution.md).
They were produced without executing the ordinary workers. The manifest records
whole-file and section identities separately and each selected function's exact
symbol extent, raw-byte SHA-256, address, static instruction count, stack operand
count, call sites and resolved indirect calls to Emuella-owned functions.

Given the two ordinary worker ELF files identified in the finding, replay with
Python 3 and GNU binutils:

```sh
python3 docs/evidence/classic-forward53-serial-attribution/extract.py \
  /path/to/baseline/classic-compare-worker \
  /path/to/candidate/classic-compare-worker \
  /path/to/replayed-extracts
```

`extract.py` is a bounded analysis recipe, not production tooling or a general
binary-equivalence checker. It asserts ELF64 little-endian input, required symbol
presence, contiguous instruction addresses and exact decoded byte coverage of
every selected symbol. Baseline `prepare` is the sole permitted absent symbol.
The output does not embed input paths. The exact replay needs these binary
identities and tool versions; rebuilding the same source does not promise them.

Each `.asm.txt` retains linked addresses, instruction bytes, register names, original
branch displacements and resolved symbol annotations. Only the input filename
banner is removed. External call labels appear as references; external function
bodies are outside the extraction. `worker_encode` is Emuella's own adapter
function, including its black-box adapter call boundary, not external codec code.

Each `.normalised.txt` is deliberately a weaker comparison:

- Instruction addresses and encoding bytes are omitted.
- Direct destinations inside the selected function become instruction indices;
  other direct destinations retain their demangled symbol names.
- RIP-relative displacements and their comments are masked as `RIP_DATA`.
  This masks data and indirect-call target identities; it does not prove them
  equal. Selected own-code indirect targets are separately resolved in JSON.
- Whitespace is collapsed. Registers, stack offsets, immediate constants,
  instruction order and padding instructions are retained.

Consequently a matching normalised hash means that this transformed text is
identical, not that the machine code, referenced data, runtime branch behaviour
or timing is identical. Non-matching hashes can reflect padding/register/layout
changes as well as generated operations. Use the raw extracts to investigate
those differences. Static stack operand counts include `lea` and error paths;
they must not be read as executed traffic, spills or a time estimate.

`static.json` hashes `.text`, `.rodata`, `.data.rel.ro`, `.eh_frame`,
`.gcc_except_table`, `.debug_line` and `.debug_info` independently. No raw section
contents or executable files are committed. This archive concerns reconstructed
binaries; the historical files and their section identities are unavailable.
