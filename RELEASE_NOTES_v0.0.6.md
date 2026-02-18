# Release: v0.0.6

## Highlights
- **New Generators**: Added dedicated news sitemap and tag generators with enhanced XML support
- **Performance Boost**: Navigation generation time significantly reduced through algorithm optimizations
- **Enhanced Benchmarking**: Comprehensive performance testing infrastructure with Criterion integration
- **Refactored Navigation**: Improved hierarchical structure support and menu generation capabilities

## Breaking Changes
- **Deprecated Module Removal**: Several modules have been deprecated and replaced with enhanced generators
  - `modules::manifest` → `generators::manifest`
  - `modules::news_sitemap` → `generators::news_sitemap`
  - `modules::tags` → `generators::tags`

## Upgrade Instructions
1. Update your Cargo.toml dependency: `staticdatagen = "0.0.6"`
2. Replace deprecated module imports:
   ```rust
   // Old
   use staticdatagen::modules::{manifest, news_sitemap, tags};

   // New
   use staticdatagen::generators::{manifest, news_sitemap, tags};
   ```
3. Run `cargo build` to verify compatibility
4. Update any custom code that relied on the old module interfaces

## Known Issues
- None reported at time of release

## Dependencies Updated
- `comrak`: 0.34 → 0.35 (improved Markdown processing)

## Performance Benchmarks
- Navigation generation: ~40% performance improvement
- File processing: Enhanced efficiency in compiler service
- New benchmarking suite available via `cargo bench --bench criterion_benchmark`

## Security Enhancements
- RFC 9116 compliance improvements
- Enhanced security utility validation mechanisms

---

**Full Changelog**: [CHANGELOG.md](./CHANGELOG.md)

> 🎨 Designed by **[Sebastien Rousseau](https://sebastienrousseau.com/)**
> 🚀 Engineered with **[Euxis](https://euxis.co/)** — Enterprise Unified eXecution Intelligence System