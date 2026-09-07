# Security Policy

## Supported Versions

| Version | Supported |
|:--------|:---------:|
| 0.0.x   | Yes       |

## Reporting a Vulnerability

Report security vulnerabilities by emailing **sebastian.rousseau@gmail.com**.

Do not open a public issue for security reports.

Include:

- A description of the vulnerability.
- Steps to reproduce.
- Affected versions.
- Any suggested fix (optional).

Expect an initial response within 48 hours. A fix or mitigation plan will follow within 7 days of confirmation.

## Security Design

staticdatagen reads content an author writes and writes a whole site to
disk. Path handling and template rendering are the two surfaces that
matter, because both decide what ends up in files a server will serve.

- `#![forbid(unsafe_code)]` — zero unsafe blocks, guaranteed by the
  compiler.
- No C dependencies, no FFI, no network I/O.

### Path safety

`utilities::security::sanitize_path` is the boundary for every path the
compiler touches. It rejects empty paths, `..` traversal, absolute paths
where a relative one is expected, and paths that escape the directory
they are resolved against. `validate_directory` checks that a directory
is usable for its stated purpose before anything is written into it.

`fuzz_path` asserts two properties beyond not panicking: a path the
function accepts never contains a `..` component, and sanitising an
already-sanitised path returns it unchanged, so a second pass cannot
smuggle anything past the first.

### Generated output

Content is rendered through `staticweaver` templates. Values that reach
a template come from the author's own front matter, not from a visitor,
so the threat model is a mistake rather than an attacker — but the HTML
post-processor still runs over generated markup with the same escaping
the rest of the family uses.

Directory listings are walked with `WalkDir::sort_by_file_name`, so two
builds of the same content produce byte-identical output on any
filesystem. That is a correctness property rather than a security one,
and it is what makes a golden-file comparison meaningful.

### Fuzzing

`fuzz/` holds three libFuzzer targets: content preprocessing and
plain-text extraction, HTML post-processing, and path sanitisation. All
three walk strings by hand rather than through a parser, which is where
byte-boundary panics live, and a panic mid-build leaves a site half
written. A committed seed corpus and every fixed-bug reproducer replay
on each push; see [`DEVELOPMENT.md`](DEVELOPMENT.md).

### Supply Chain### Supply Chain

- `cargo-deny` (licences, advisories, sources) and `cargo-audit` in CI.
- Dependency provenance recorded with `cargo-vet` (`supply-chain/`);
  exemptions are regenerated on every dependency change and the CI
  ratchet lets the count shrink but never grow.
- The first-party crates it builds on — `html-generator`,
  `metadata-gen`, `frontmatter-gen`, `staticweaver`, `langweave`,
  `sitemap-gen` — are trusted in `cargo-vet` by publisher, and a bump is
  a deliberate release of this crate with its own changelog entry.
- Test-only crates live in `[dev-dependencies]`, so a consumer's build
  does not pull them.
- `Cargo.lock` committed for deterministic builds; CI builds `--locked`.
- All GitHub Actions SHA-pinned.
- REUSE/SPDX compliance linted in CI.

### Commit Integrity

All commits on the main branch are signed, and releases are signed
tags. The release-signing key is published in [`KEYS.asc`](KEYS.asc):

```text
4B7F16C909C7A8EE9BED338A4F047EDF5F90F638
```

Signing key `Sebastien Rousseau <sebastian.rousseau@gmail.com>`,
ed25519, signing-only, expires 2028-08-16. Verify the fingerprint out
of band before trusting it.
