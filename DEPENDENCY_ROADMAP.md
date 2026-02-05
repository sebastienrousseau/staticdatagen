# Dependency Stability Roadmap

This document outlines the stability status and roadmap for dependencies used in `staticdatagen`.

## Overview

The `staticdatagen` library relies on a mix of stable ecosystem crates and custom early-stage crates. This document provides transparency about dependency maturity and plans for stabilization.

## Dependency Classification

### Stable Dependencies (1.x+)

These dependencies are mature and production-ready:

| Crate | Version | Purpose |
|-------|---------|---------|
| `anyhow` | 1.0 | Error handling |
| `clap` | 4.5 | CLI argument parsing |
| `comrak` | 0.35 | CommonMark rendering |
| `idna` | 1.0.3 | Internationalized domain names |
| `lazy_static` | 1.5 | Lazy static initialization |
| `log` | 0.4 | Logging facade |
| `minify-html` | 0.15 | HTML minification |
| `pulldown-cmark` | 0.12 | Markdown parsing |
| `quick-xml` | 0.37 | XML processing |
| `rayon` | 1.10.0 | Parallel processing |
| `regex` | 1.11.1 | Regular expressions |
| `serde` | 1.0 | Serialization |
| `serde_json` | 1.0 | JSON serialization |
| `tempfile` | 3.14 | Temporary file handling |
| `thiserror` | 2.0 | Error derive macros |
| `time` | 0.3 | Date/time handling |
| `toml` | 0.8 | TOML parsing |
| `url` | 2.5 | URL parsing |
| `uuid` | 1.11 | UUID generation |
| `xml-rs` | 0.8 | XML reading |

### Early-Stage Custom Dependencies (0.0.x)

These are custom crates that are part of the same ecosystem and are actively maintained:

| Crate | Version | Purpose | Stability Plan |
|-------|---------|---------|----------------|
| `dtt` | 0.0.9 | DateTime utilities | Target 0.1.0 Q2 2026 |
| `html-generator` | 0.0.3 | HTML generation | Target 0.1.0 Q2 2026 |
| `http-handle` | 0.0.2 | HTTP server utilities | Target 0.1.0 Q3 2026 |
| `langweave` | 0.0.1 | Internationalization | Target 0.1.0 Q2 2026 |
| `metadata-gen` | 0.0.1 | Metadata generation | Target 0.1.0 Q2 2026 |
| `rlg` | 0.0.6 | Logging utilities | Target 0.1.0 Q2 2026 |
| `rss-gen` | 0.0.3 | RSS feed generation | Target 0.1.0 Q2 2026 |
| `sitemap-gen` | 0.0.1 | Sitemap generation | Target 0.1.0 Q2 2026 |
| `staticweaver` | 0.0.1 | Template weaving | Target 0.1.0 Q2 2026 |
| `vrd` | 0.0.8 | Random number generation | Target 0.1.0 Q2 2026 |

## Risk Assessment

### Low Risk
- All early-stage dependencies are maintained by the same team
- Breaking changes are coordinated across the ecosystem
- Comprehensive CI/CD ensures compatibility

### Mitigation Strategies

1. **Version Pinning**: All dependencies use exact version requirements
2. **cargo-deny**: Advisory database checks catch security issues
3. **Coordinated Releases**: Custom crates are updated in lockstep
4. **Feature Isolation**: Optional features reduce exposure to unstable deps

## Stabilization Timeline

### Phase 1: Q2 2026
- Stabilize core utilities: `dtt`, `rlg`, `vrd`
- Release 0.1.0 versions with semantic versioning commitment

### Phase 2: Q3 2026
- Stabilize content generators: `html-generator`, `rss-gen`, `sitemap-gen`, `metadata-gen`
- Release 0.1.0 versions

### Phase 3: Q4 2026
- Stabilize remaining crates: `http-handle`, `langweave`, `staticweaver`
- Prepare `staticdatagen` 1.0.0 release

## Recommended Usage

### For Production Use
If you require maximum stability, consider:

1. **Pin exact versions** in your `Cargo.lock`
2. **Test thoroughly** before updating dependencies
3. **Monitor the changelog** for breaking changes

### For Development/Testing
The current dependency configuration is suitable for:

- Development and testing environments
- Non-critical applications
- Projects that can tolerate occasional updates

## Monitoring

Dependencies are monitored via:

- **Dependabot**: Automated security updates
- **cargo-deny**: License and advisory checks
- **CI/CD**: Continuous compatibility testing

## Questions or Concerns?

If you have questions about dependency stability, please:

1. Open an issue on [GitHub](https://github.com/sebastienrousseau/staticdatagen/issues)
2. Review the [CHANGELOG](CHANGELOG.md) for update history
3. Check individual crate documentation for specific stability guarantees
