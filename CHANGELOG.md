# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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