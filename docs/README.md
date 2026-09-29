# Documentation

Start with the [README](../README.md) for project status and a first inspection.
The Rust facade is the application-facing API; supported inputs and output
requests remain bounded while the project prepares its first release.

## Choose an operation

- [Supported profiles](supported-profiles.md) maps inspection, decoding and
  encoding to containers, component formats, output modes and important limits.
- [Decoder profile contracts](decoding-profiles.md) gives exact admission and
  request boundaries for selected Part 1 and HTJ2K routes.
- [Native planes](native-planes.md), [JP2 mapped presentation](jp2-presentation.md)
  and [Part 1 precision](part1-precision.md) explain sample and presentation
  behaviour.
- [Scalable lossless encoding](scalable-lossless.md),
  [eight-component coding](native-eight-components.md),
  [rate control](encoder-rate-control-calibration.md) and
  [lossy HTJ2K](ht-lossy-public-api.md) contain encoder limits and evidence.
- [C ABI safety contract](c-abi-safety-contract.md) covers the experimental
  native interface.

## Integrate or contribute

- [Testing](testing.md) distinguishes the self-contained gate from opt-in
  corpus qualification and records selected evidence.
- [Architecture](architecture.md) describes crate ownership and codec mechanisms.
- [Contributing](../CONTRIBUTING.md) provides setup, verification and
  documentation guidance; [release preparation](releasing.md) is separate from
  everyday development.

Qualification results in these documents apply to their stated inputs and
revisions. They do not imply support for every JPEG 2000 or HTJ2K profile.
