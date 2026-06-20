# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.9] — 2026-06-21

### Changed
- **Dependencies** — Bumped `html-generator` to 0.0.6, `metadata-gen` to 0.0.3, and `sitemap-gen` to 0.0.2. The transitive chain now consumes `noyalib` (pure-Rust, zero unsafe) instead of `serde_yml`/`libyml`, dropping the unmaintained C-FFI YAML stack and resolving RUSTSEC-2025-0067 / RUSTSEC-2025-0068 for downstream consumers.

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