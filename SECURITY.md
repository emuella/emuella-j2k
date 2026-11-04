# Security policy

Please report suspected vulnerabilities privately to the project maintainer
before opening a public issue. Include the affected version, a minimal input or
reproducer when safe to share, and the expected security impact.

JPEG 2000 inputs are untrusted data. Parser and decoder changes must retain
checked length arithmetic, bounded allocation, explicit unsupported cases, and
fuzz coverage. Decode allocation is bounded by built-in sample caps, which
still permit hundreds of megabytes for a tiny input that declares a large
image. Applications that decode untrusted input should lower the bound with
`DecodeLimits::max_output_samples`, checked from headers before decode work;
see [caller decode limits](docs/rust-api.md#caller-decode-limits). Tests must not invoke downloaded binaries or materialize an
external corpus without an explicit operator action.
