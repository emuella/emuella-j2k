# emuella-j2k

`emuella-j2k` is a pure-Rust JPEG 2000 and HTJ2K library for inspecting images,
encoding selected image profiles, and decoding pixels through an
application-facing Rust API. It includes a small command-line inspection tool.
Use it when you need to examine a codestream or container, work with native
component samples, or encode and decode within the documented profiles.
Structural inspection and pixel reconstruction have different admission rules:
an image may have readable metadata while a particular decode request remains
unsupported. Native component output also preserves codestream sample meaning
instead of applying a display colour interpretation.

**Development status:** The project is preparing its first public release.
Version `0.1.0` is present in the source tree but is not published on crates.io.
The public facade is the intended entry point for applications; its APIs and
supported profiles may still change. Support is deliberately bounded and does
not imply general JPEG 2000, HTJ2K or JP2 conformance. Start with the
[supported-profile guide](https://github.com/emuella/emuella-j2k/blob/main/docs/supported-profiles.md)
when deciding whether a particular input and output request is covered.

## What you can do

- Inspect raw J2K and HTJ2K codestreams or JP2 and JPH containers, including
  metadata and a decode-support classification.
- Decode selected classic Part 1 and HTONLY profiles as rendered pixels or
  native components, according to the input and request. Selected JP2 palettes,
  channel mappings and straight alpha produce rendered U8 output.
- Encode selected lossless and target-rate Part 1 or HTJ2K profiles. Available
  raw codestream and JP2/JPH outputs depend on the encoder profile.
- Request component, reduced-resolution or regional output for selected
  profiles. These routes have distinct input, geometry and output limits; see
  the [decoder contracts](https://github.com/emuella/emuella-j2k/blob/main/docs/decoding-profiles.md).

## Get started

Install Git and the repository's pinned Rust toolchain (currently Rust 1.98.1).
Until a crate is published, add the facade to your application's `Cargo.toml`
from source:

```toml
[dependencies]
emuella-j2k = { git = "https://github.com/emuella/emuella-j2k", branch = "main" }
```

The package name uses a hyphen; Rust imports use `emuella_j2k`. This example
inspects a JPEG 2000 or HTJ2K file you supply and prints its declared image
size, component count and current decode-support classification. Inspection
does not reconstruct pixels, and a reported input may still be outside a
particular decode request.

```rust
use emuella_j2k::{inspect, InspectOptions};
use std::{env, fs, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "usage: inspect-image IMAGE.jp2")
    })?;
    let bytes = fs::read(path)?;
    let metadata = inspect(&bytes, &InspectOptions::default())?;
    if let Some(image) = &metadata.image {
        println!("{} × {}; components: {}", image.width, image.height, image.components);
    }
    println!("format: {:?}; decode support: {:?}", metadata.format, metadata.support);
    Ok(())
}
```

Run it as `cargo run -- path/to/image.jp2` in your application. The repository
also contains the `emuella-j2k-cli` package. From a source checkout, run its
inspection command against your own file:

```sh
git clone https://github.com/emuella/emuella-j2k.git
cd emuella-j2k
cargo run --release -p emuella-j2k-cli -- inspect path/to/image.jp2
```

Both examples accept a local input supplied by you; no private corpus or test
fixture is required. For pixel decoding and encoding, choose options from the
[profile guide](https://github.com/emuella/emuella-j2k/blob/main/docs/supported-profiles.md)
and the public Rust API before assuming an input is admitted.

## Documentation and help

The [documentation index](https://github.com/emuella/emuella-j2k/blob/main/docs/README.md)
leads to user and integrator contracts, qualification evidence, architecture
and testing. In particular, see [native components](https://github.com/emuella/emuella-j2k/blob/main/docs/native-planes.md),
[JP2 presentation](https://github.com/emuella/emuella-j2k/blob/main/docs/jp2-presentation.md),
[lossless encoding](https://github.com/emuella/emuella-j2k/blob/main/docs/scalable-lossless.md),
[lossy HTJ2K](https://github.com/emuella/emuella-j2k/blob/main/docs/ht-lossy-public-api.md)
and the [experimental C ABI](https://github.com/emuella/emuella-j2k/blob/main/docs/c-abi-safety-contract.md).
Use [GitHub issues](https://github.com/emuella/emuella-j2k/issues) for a
reproducible problem or support question, with the input profile and requested
output mode where possible.

## Contributing and licence

[CONTRIBUTING.md](https://github.com/emuella/emuella-j2k/blob/main/CONTRIBUTING.md)
covers source provenance, local verification and documentation updates.
Project-authored material is Apache-2.0. Isolated HTJ2K modules contain
OpenJPH-derived code or table data under BSD-2-Clause; consult
[NOTICE](https://github.com/emuella/emuella-j2k/blob/main/NOTICE),
[THIRD_PARTY.md](https://github.com/emuella/emuella-j2k/blob/main/THIRD_PARTY.md)
and the [BSD licence text](https://github.com/emuella/emuella-j2k/blob/main/LICENSES/OpenJPH-BSD-2-Clause.txt)
before redistribution.
