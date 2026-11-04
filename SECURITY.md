# Security policy

Please report suspected vulnerabilities privately to the project maintainer
before opening a public issue. Include the affected version, a minimal input or
reproducer when safe to share, and the expected security impact.

JPEG 2000 inputs are untrusted data. Parser and decoder changes must retain
checked length arithmetic, bounded allocation, explicit unsupported cases, and
fuzz coverage. Tests must not invoke downloaded binaries or materialize an
external corpus without an explicit operator action.

Decode allocation is bounded by built-in sample caps. Those caps still let a
tiny input that declares a large image use a gigabyte or more of memory.
Applications that decode untrusted input through the Rust API should lower the
output bound with `DecodeLimits::max_output_samples`, which is checked from
headers before decode work; see
[caller decode limits](docs/rust-api.md#caller-decode-limits). The limit counts
output samples; internal working storage on some routes can exceed it. The C
API, Python and CLI adapters cannot set this limit yet and keep the built-in
caps.
