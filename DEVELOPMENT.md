<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Developing staticdatagen

The single entry point for working on this repository. User-facing
documentation lives in the [README](README.md) and [`docs/`](docs/);
contribution etiquette and review expectations live in
[`CONTRIBUTING.md`](CONTRIBUTING.md). This file is the *how*: toolchain,
tasks, and reproducing every CI gate locally.

## Toolchain

| What | Version | Why |
| :--- | :--- | :--- |
| Rust stable | `rust-version` in `Cargo.toml` or later | MSRV, enforced by Cargo and CI |
| Rust nightly | any recent | Miri, cargo-fuzz, coverage (`cargo-llvm-cov`) |
| cargo-deny, cargo-vet, cargo-audit | cargo-vet **0.10.2 or later** | supply-chain gates; earlier cargo-vet rejects a `trusted-publisher` entry |
| cargo-llvm-cov | latest | coverage gate |
| cargo-fuzz | latest, **installed from source** (`cargo install --locked cargo-fuzz`) | the prebuilt musl binary infers its own build triple as the fuzz target and dies on "sanitizer is incompatible with statically linked libc" |
| uv (`uvx`) and npx | any | `reuse`, `codespell`, `markdownlint` for the docs lint |

```bash
git clone https://github.com/sebastienrousseau/staticdatagen
cd staticdatagen
make            # check + clippy + test — the default gate
```


## Task map

| Task | Command |
| :--- | :--- |
| Everything a PR needs first | `make` |
| Full test suite | `make test` |
| Lints / formatting | `make clippy` / `make fmt` |
| Docs lint (markdownlint, codespell, REUSE) | `make lint` |
| Docs as CI builds them | `make doc` |
| Coverage gate | `make coverage` |
| Miri | `make miri` |
| Fuzz targets, corpus replay | `make fuzz` |
| All examples | `make examples` |
| Benches compile and run once | `make bench-smoke` |
| Version-bearing files agree | `make versions` |
| Supply chain | `make deny` / `make vet` / `make audit` |

## Reproducing the CI gates

CI has two workflows. [`ci.yml`](.github/workflows/ci.yml) calls the
shared pipelines from
[`sebastienrousseau/pipelines`](https://github.com/sebastienrousseau/pipelines);
[`quality.yml`](.github/workflows/quality.yml) holds the gates the
repository standard requires that the shared pipeline does not cover.

| CI job | Local reproduction | Gotcha |
| :--- | :--- | :--- |
| `ci` (fmt, clippy, test, cross-platform) | `make` | |
| `coverage-gate` | `make coverage` | nightly; 98 % lines, no exclusions |
| `miri` | `make miri` | lib tests; filesystem tests carry `#[cfg_attr(miri, ignore)]` |
| `fuzz-replay` | `make fuzz` | needs a source-installed cargo-fuzz (see above) |
| `docs-lint` | `make lint` | British spellings are house style; see `.codespellrc` |
| `cargo-vet` | `make vet` | after a dep change: `cargo vet regenerate exemptions`; the count must not exceed `supply-chain/exemptions-baseline.txt` |
| `release-hygiene` | `make versions && make examples && make bench-smoke && make doc` | |
| `cargo-audit` | `make audit` | if a local cargo alias named `audit` shadows the subcommand, run `cargo-audit audit` |

## Coverage: the threshold and why

The gate is **98 % lines**, measured with `cargo llvm-cov
--all-features`, with **no exclusions**. The crate is a library with no
binary entry point and no target-gated glue, so every line is reachable
from a test.

What remains uncovered is defensive: error arms for conditions the
surrounding code has already excluded, and a handful of I/O failure
paths that would need a filesystem fault to reach.

## Test layout

- `src/**` `#[cfg(test)]` — unit tests next to the code.
- `tests/integration_tests.rs` and `tests/compiler_integration.rs` —
  the compiler driven end to end over a temporary site.
- `examples/` — 25 runnable examples, all run in CI.
- `benches/` — three Criterion harnesses (`criterion_benchmark`,
  `compile_scaling`, `performance_stress_test`), smoke-run in CI.
- `fuzz/fuzz_targets/` — `fuzz_markdown` (content preprocessing and
  plain-text extraction), `fuzz_html` (post-processing) and `fuzz_path`
  (path sanitisation, which also asserts that an accepted path has no
  `..` component and that sanitising is idempotent).
  `fuzz/corpus/<target>` is the committed seed set and
  `fuzz/regressions/<target>` holds every fixed-bug reproducer. Both
  replay per push.

## Release model

Versions increment strictly by `+0.0.1`. Before tagging, run
`make versions`: it checks `Cargo.toml`, `Cargo.lock`, the first-party
dependency versions, `CITATION.cff`, the `CHANGELOG.md` heading and every
install snippet. Tags are signed (`git tag -s vX.Y.Z`); the key is in
[`KEYS.asc`](KEYS.asc).

`release.yml` is tag-triggered and publishes to crates.io; pushing a
signed `vX.Y.Z` tag is the whole release.

## House rules

- CI must be green in the same session that turned it red.
- Commits are signed; releases are signed tags.
- Structure cleanups never couple to code changes.
- New behaviour lands with its test in the same commit; a regression
  fix lands with the input that found it.
