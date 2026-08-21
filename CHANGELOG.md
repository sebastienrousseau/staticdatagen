# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.14] — 2026-08-21

A documentation-and-tooling pass. No behaviour changes beyond one
deprecation; the value is in what turned out not to be true.

### Changed

- **`server` is no longer a default feature.** It pulls `http-handle`, which
  is `license = "AGPL-3.0-only"`, while this crate is published as
  Apache-2.0 OR MIT — so every default build handed its consumer an AGPL
  transitive. `ssg` was one of them: it depends on this crate with default
  features and never references the re-exported `Server`.

  The README promised this exit for v0.0.12 ([#83]) and it did not happen.
  Removing `server` from `default` is the part achievable without replacing
  the dependency; replacing it remains open.

  **Breaking if you used `staticdatagen::Server` with default features.**
  Add `features = ["server"]`, or use `full`, which still includes it. A
  default build now resolves `http-handle` zero times, verified with
  `cargo tree --edges normal`.

### Deprecated

- **`move_output_directory`.** It hardcoded `public/`, resolved against the
  caller's working directory, and the first thing it did there was
  `remove_dir_all`. Every test in the module called the private
  `move_output_directory_to` instead, precisely so a test run could not
  delete the repository's own `public/` — an API its own tests avoid is not
  one to hand a consumer. `move_output_directory_to` is now public and takes
  the destination as an argument.

### Added

- **`benches/compile_scaling.rs`.** `compile` had never been benchmarked,
  despite being the function the README quotes a figure for and the target
  of the 0.0.12 parallel work. Covers 12 pages (sequential, below
  `PARALLEL_THRESHOLD`) and 50/200/500 (parallel). It asserts the compile
  succeeded: a failed `compile` returns in microseconds, and the first draft
  of this benchmark timed exactly that without noticing.
- **Documented output layout** for `move_output_directory_to`: the moved
  directory keeps its own name (`public/site/out/`, not `public/site/`), and
  spaces in `site_name` become underscores. Both were silent before.

### Fixed

- **`performance_stress_test` was never compiled.** 360 lines of benchmarks
  with no `[[bench]]` entry in `Cargo.toml`, so cargo ignored the file
  entirely. It has been in that state since before 0.0.10 and could not have
  reported a regression.
- **`static_site_example` never terminated.** It started a blocking server
  unconditionally, so `cargo run --example static_site_example` hung, the
  line telling you where to look was unreachable, and a sweep over every
  example left an orphan holding port 3000. Serving is now opt-in via
  `STATICDATAGEN_EXAMPLE_SERVE=1`.
- **`static_site_example` wrote into the source tree.** Output went to
  `examples/site` and `examples/build`; `examples/site` is gitignored, so
  `git status` — the usual check for this — could not see it. Outputs now go
  to `target/`, matching what #116 did for `service_example`.

### Documentation

The README described 0.0.10 and made claims that had stopped being true:

- The capabilities section still tracked 0.0.10, four releases behind.
- Test counts read "714 lib tests, 55 doctests"; the real figures are 754
  lib, 61 doc (3 ignored) and 18 integration.
- The performance claim — "a 500-page build went from 2.83 s to under
  0.6 s" — was not reproducible from this repository, because nothing
  benchmarked `compile`. It is replaced with measured figures and the
  command to regenerate them.
- Three roadmap promises had silently expired: the AGPL exit ([#83]) was
  slated for v0.0.12 and `http-handle` is still a dependency; the
  incremental cache ([#87]) was slated for v0.0.13 and no incremental code
  exists; `cargo vet` ([#76]) was slated for v0.0.11 and there is no
  `supply-chain/` directory. The AGPL one mattered most — a reader checking
  licence exposure would have concluded it was gone.

The roadmap table now says plainly that it is a plan rather than a record.

## [0.0.13] — 2026-08-20

Stops asking the HTML pipeline for structured data it could never produce.

`generate_structured_data` was enabled for every Markdown body. It reads a
`<title>`, and a Markdown body is a fragment with no `<head>` — so the step
failed on every page ever compiled, logging an error each time. A themes
showcase build reported it 25 times, once per page.

Nothing was lost when it failed and nothing is gained by fixing it here:
structured data is generated downstream from front matter, where the title,
description, canonical URL and page type are known, and that block is a
full `@graph` rather than the bare name/description derivable from a
fragment. Enabling it would emit a second, thinner `ld+json` block
competing with the real one.

Output is unchanged. What goes away is 25 error diagnostics per build and a
step that never once did anything.

## [0.0.12] — 2026-08-17

Performance release. `compile` renders and writes pages in parallel, which
takes a 500-page build from 2.83 s to under 0.6 s — at least 67% faster at
every size measured — with output unchanged.

### Changed

- **`compile` renders and writes in parallel.** Profiling a 500-page corpus
  put 94% of wall-clock inside this function, and each page is independent:
  it reads shared, immutable navigation and writes only its own output. Two
  things made the loop sequential — a `&mut Engine` and a `&mut HashMap` of
  tags. Each rayon worker now holds its own `Engine`, so the template cache
  is per-thread rather than contended, and each file reports the tags it
  contributes for merging afterwards. The merge iterates the collected
  results in order, so output does not depend on thread scheduling.

  Measured on an M-series laptop, medians of 3, each build in isolation:

  | Pages | 0.0.11 | 0.0.12 | Faster |
  |-------|--------|--------|--------|
  |    10 |   45.4 ms |   15.1 ms | 66.8% |
  |   100 |  387.7 ms |  127.6 ms | 67.1% |
  |   500 | 2,830 ms  |  578.2 ms | 79.6% |

  The 500-page figure moves between roughly 425 ms and 615 ms depending on
  machine load; the table quotes the slower sample rather than the best
  one. The direction is stable across every run: never worse than 67%.

- **Small sites stay on the calling thread.** Below `PARALLEL_THRESHOLD`
  (24 pages) both the render and write loops run sequentially: starting
  rayon's pool costs more than the parallelism saves. A 10-page build
  measured 35 ms sequentially against 158 ms parallel before this guard was
  added, and most sites are small.

### Added

- `parallel_and_sequential_render_identically` — renders the same pages
  through both branches and asserts the per-page HTML matches. Verified to
  fail when the parallel path is deliberately corrupted, rather than only
  ever having passed.
- `parallel_compilation_is_deterministic` — two parallel runs of one corpus
  must produce identical bytes, pinning the ordered tag merge.
- `compiles_either_side_of_the_parallel_threshold` — covers the boundary
  where the branch is chosen, so an off-by-one cannot silently send every
  build down one path.

### Notes

Emitted HTML is byte-identical to 0.0.11: verified across a four-theme,
34-page site as well as in-crate. Run-to-run ordering differences in
`.meta/*.json` and `depgraph.json` are pre-existing `HashMap` iteration
order — confirmed by building 0.0.11 twice and observing the same
differences — and are not affected by this change.

## [0.0.11] — 2026-07-25

Correctness release driven by the ssg v0.0.47 plan (ssg#586, spec
items A1/A2/A4 of the ssg fixes-and-native-migration specification).

### Fixed
- **A1 — Raw HTML in Markdown was escaped (P0).** `html-generator 0.0.6` introduced `HtmlConfig.allow_unsafe_html` defaulting to `false`, which escaped raw block HTML (`<section>`, `<figure>`, inline `<svg>`, …) in Markdown bodies. `compiler::service::generate_html_content` now explicitly opts into pass-through (`allow_unsafe_html: true`) — the trusted-author default; sanitisation remains an explicit opt-in (`sanitize_html`), never silent escaping. Regression test asserts `<section class="x">` renders unescaped. (`src/compiler/service.rs`)
- **A2 — `channel.link is missing` hard-failed the whole build (P0).** A page without `permalink:` front matter aborted the entire compile at RSS validation (and independently at sitemap generation). Two changes: (1) a missing `permalink` is now derived via the fallback chain `permalink` → `url` → `{base_url}/{relative_output_path}` → `base_url`, so a correct feed/sitemap link is always available when the site provides its base URL; (2) a genuinely underivable feed or sitemap entry logs a warning and is skipped — it never aborts the compile. Authors no longer need to hand-write `permalink`. (`src/compiler/service.rs`)
- **A4 — news-sitemap date parser rejected common formats (P1).** `generators::news_sitemap` accepted only RFC 2822, spamming `Parsing failed: the 'day' component could not be parsed. Using fallback.` for front matter written as `July 1, 2026` or `2026-07-01`. Date parsing is now routed through the new `utilities::dates::parse_flexible_date`, which accepts, in order: RFC 2822, long form, and ISO 8601 (date or datetime). The fallback to the current time survives as a last resort and now logs the failing field and every attempted format. (`src/generators/news_sitemap.rs`)

### Added
- **`utilities::dates`** — dependency-free flexible date parsing (RFC 2822 / long form / ISO 8601) with deterministic, locale-independent output formatting (`to_rfc2822`, `to_rfc3339`, `to_w3c_date`, `to_iso_date`), ported from ssg's `src/core/dates.rs` so both layers agree on what a date means. Includes property tests round-tripping dates 1990–2100 through all three spec formats.

## [0.0.10] — 2026-06-28

### Fixed
- **#67 — Empty `layout:` key crashed `render_page`.** Frontmatter missing or empty `layout:` now falls back to `"page"` instead of passing `""` to staticweaver (which aborted with `invalid template or partial name: ""`). Unblocks every page authored without a layout key, including the ≈ 1,137 of 2,371 affected files on multilingual Jekyll-style trees. (`src/compiler/service.rs`)
- **#68 — `copy_auxiliary_files` aborted when `main.js` / `sw.js` were absent.** The copy is now best-effort: missing auxiliary files are logged at `debug` and skipped instead of failing the build with opaque `os error 2`. Sites that don't ship a service worker can build cleanly. (`src/utilities/write.rs`)
- **#69 — `write_tags_html_to_file` aborted builds without a tags template.** Skips the substitution gracefully when `tags/index.html` is absent. (`src/generators/tags.rs`)
- **#70 — `add()` skipped subdirectories.** Switched to `walkdir::WalkDir` so multilingual `_posts/<lang>/` trees and any nested locale layout are processed. Per-locale subdirs are preserved through `get_processed_file_name` via `with_extension("")` so output URLs keep their `fr/`, `bn/`, etc. prefixes. (`src/utilities/file.rs`, `src/utilities/write.rs`)
- **#71 — Misleading "Successfully generated…" log fired before compile errors propagated.** The success line now fires *after* the final `fs::rename`, so log scrapers (`ssg`, CI tooling) can rely on it as a build-state signal. (`src/compiler/service.rs`)

### Security
- **GHSA-cq8v-f236-94qc / RUSTSEC-2026-0097** — Bumped `rand` to 0.8.6, resolving the Stacked-Borrows unsoundness in `ThreadRng` reachable through custom loggers that call `rand::rng()` while reseeding. Closes Dependabot alerts #1 and #2.

### Changed
- **Dependencies** — Bumped to current latest minors:
  - `staticweaver` 0.0.2 → 0.0.3
  - `rss-gen` 0.0.5 → 0.0.6
  - `regex` 1.11 → 1.12
  - `walkdir` 2 → 2.5
  - `uuid` 1.11 → 1.23
  - `idna` 1.0 → 1.1
  - `rayon` 1.10 → 1.12
  - `proptest` (dev) 1.6 → 1.11
  - `tempfile` (dev) 3 → 3.27

## [0.0.9] — 2026-06-21

### Changed
- **Dependencies** — Bumped `html-generator` to 0.0.6, `metadata-gen` to 0.0.4, and `sitemap-gen` to 0.0.2. The transitive chain now consumes `noyalib` (pure-Rust, zero unsafe) instead of `serde_yml`/`libyml`, dropping the unmaintained C-FFI YAML stack and resolving RUSTSEC-2025-0067 / RUSTSEC-2025-0068 for downstream consumers. (Pinning `metadata-gen 0.0.4` rather than the on-registry `0.0.3` is required — the published `0.0.3` predates the noyalib migration; only `0.0.4` carries it.)
- **`rss-gen` 0.0.3 → 0.0.5** — Picks up the upstream `dtt` 0.0.10 API fix (the private `DateTime::offset` field that was breaking Strict CI on `main`). Restores compilation under stable, nightly, MSRV (1.88.0), and the cross-platform matrix.
- **`staticweaver` 0.0.1 → 0.0.2** — Tera-tier templating engine with template inheritance, expression language, 23 built-in filters, and SIMD HTML escape. Adapted `compiler::service` to the removed `PageOptions` type (write directly into `Context`).
- **`HtmlConfig` migration** — Adapted call sites to the new fields added in `html-generator` 0.0.6 (`allow_unsafe_html`, `sanitize_html`, `generate_full_document`, `max_buffer_size`, `encoding`, `enable_math`, `enable_diagrams`) using struct-update syntax (`..HtmlConfig::default()`).
- **`actions/checkout` v4 → v7** — Updated raw checkout references in the consolidated `ci.yml` (miri and semver jobs); other workflow callers run through the shared `pipelines` repo and inherit the bump centrally.

### Absorbed (dependabot)
- #52 `pulldown-cmark` 0.12 → 0.13 (already on branch)
- #53 `staticweaver` 0.0.1 → 0.0.2
- #56 `peaceiris/actions-gh-pages` 4.0.0 → 4.1.0 (no longer referenced after workflow consolidation)
- #57 `codecov/codecov-action` 5 → 7 (no longer referenced after workflow consolidation)
- #58 `actions/checkout` 6 → 7 (applied to ci.yml)

### Closed as obsolete
- #51 `metadata-gen` 0.0.2 → 0.0.3 — superseded; branch pins `metadata-gen 0.0.4` (the on-registry `0.0.3` predates the noyalib migration, so the bump goes straight to `0.0.4`).
- #54 `vrd` 0.0.9 → 0.0.10 — `vrd` was removed entirely in 8bb3e2f as part of the dep-graph slim-down.
- #55 `html-generator` 0.0.4 → 0.0.5 — branch already at 0.0.6.

## [0.0.8] — 2026-03-11

### Added
- Integrated `euxis-commons` local dependency for shared utilities
- Updated GitHub Actions: `upload-artifact` to v7 and `download-artifact` to v8

### Changed
- **Dependencies updated** — Updated `rlg` to 0.0.8, `comrak` to 0.51, `pulldown-cmark` to 0.13, `http-handle` to 0.0.4, and `langweave` to 0.0.2
- **Logging modernization** — Updated `macro_log_info!` to use the new `rlg` 0.0.8 builder API and fire-and-forget pattern
- **CI/CD hardening** — Enhanced release and strict-ci workflows with latest artifact actions

## [0.0.7] — 2026-02-16

### Added
- Comprehensive unit test coverage reaching 95%+ across all metrics (#418 tests)
- 64 new unit tests including compile and process_file tests to close coverage gaps
- Stress benchmarks for performance monitoring and regression detection
- Enhanced feature gates for modular compilation and dependency optimization

### Changed
- **Error handling modernized** — Unified error construction patterns across codebase for consistency
- **Clippy lint compliance enforced** — All clippy lints resolved with `unwrap_used` and `expect_used` denied for production readiness
- **Dependencies updated** — All dependencies bumped to latest versions for security and performance
- **Performance optimized** — Error construction patterns modernized for reduced allocation overhead
- Process file functionality split into focused helper functions for maintainability

### Fixed
- **Security hardening** — Addressed all deep-review security findings with enhanced validation
- **Cross-platform support** — Improved logging and platform compatibility
- **License consistency** — Unified license headers across all source files
- 24 unused-result warnings suppressed in tests and benchmarks
- Project name correction in Rust version error messages
- Clippy lints fully resolved across codebase

### Security
- **MEDIUM severity** — Enhanced input validation and path sanitization
- **LOW severity** — Dependency security audit completed with warnings noted
- Hardened security utilities with improved cross-platform support

## [0.0.6] — 2026-02-05

### Added
- New `news_sitemap.rs` generator for generating news sitemaps with comprehensive XML support
- New `tags.rs` generator for enhanced tag management and categorization
- Comprehensive benchmarking suite with `criterion_benchmark.rs` for performance testing
- Enhanced service compiler with improved file processing capabilities

### Changed
- **Navigation system refactored** — Significant improvements to `src/modules/navigation.rs` with enhanced menu generation and hierarchical structure support
- **Service compiler enhanced** — Major updates to `src/compiler/service.rs` with improved file processing loop and performance optimizations
- Library core (`src/lib.rs`) substantially expanded with new functionality and improved documentation
- Updated dependency: comrak from 0.34 to 0.35 for improved Markdown processing
- Updated minimum Rust version requirements and build configuration

### Removed
- Deprecated `src/modules/manifest.rs` — functionality migrated to generators
- Deprecated `src/modules/news_sitemap.rs` — replaced with enhanced generator version
- Deprecated `src/modules/tags.rs` — replaced with enhanced generator version
- Removed obsolete `cname_benchmark.rs` — replaced with comprehensive criterion benchmarks

### Fixed
- Documentation formatting issues in `build.rs` with proper indentation
- Unused import warnings and lint configuration cleanup
- Missing fragment specifier lint warnings resolved

### Security
- Enhanced security review compliance for RFC 9116 standards
- Improved security utilities with updated validation mechanisms

### Migration Guide

If you were using the removed modules:
- Replace `modules::manifest` usage with `generators::manifest`
- Replace `modules::news_sitemap` usage with `generators::news_sitemap`
- Replace `modules::tags` usage with `generators::tags`

The new generators provide enhanced functionality while maintaining backward compatibility for most use cases.

### Performance Improvements
- Navigation generation time significantly reduced through algorithm optimizations
- Enhanced file processing efficiency in the service compiler
- New benchmarking infrastructure for continuous performance monitoring

---

*Engineered with [Euxis](https://euxis.co/) — Enterprise Unified eXecution Intelligence System*