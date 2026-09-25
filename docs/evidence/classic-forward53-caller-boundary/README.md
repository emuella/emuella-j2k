# Prospective three-arm compiled evidence

The [source record](../../forward53-caller-boundary.md) defines A, B and C.
This wrapper imports the unchanged [bounded attribution extractor](../classic-forward53-serial-attribution/extract.py)
and its exact-byte coverage checks and limited normalisation. It adds C's
owning helper to the selected Emuella symbols and records known absent
preparation/owning-helper symbols explicitly. The owning component call also
resolves its register-loaded target through the ELF relocation slot. It never
executes a worker or
extracts an external implementation body.

```sh
python3 docs/evidence/classic-forward53-caller-boundary/extract.py \
  /path/to/A/classic-compare-worker \
  /path/to/B/classic-compare-worker \
  /path/to/C/classic-compare-worker \
  /path/to/replayed-extracts
```

The files retain each selected symbol's linked addresses and instruction bytes,
frame stack subtractions, exact raw and normalised hashes, selected call targets
and section identities. The original extractor's [evidence guide](../classic-forward53-serial-attribution/README.md)
defines all masking: instruction-index internal branches, demangled external
labels, masked RIP-relative displacement/data references and collapsed
whitespace. Matching normalised text does not prove equal placement, referenced
data, executable bytes or runtime behaviour. Stack operands include locals,
address formation, arguments and error paths; they are not executed traffic,
a spill count or a time estimate.

These are prospective rebuilt treatments. Historical binary identity is not
established. Linked code and layout changes elsewhere limit isolation of the
narrow caller-state mechanism; the measured source-level refactoring remains
the intervention. No source or build selection follows from these observations.
