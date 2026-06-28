# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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