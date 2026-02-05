# Roadmap to 1.0

This document outlines the planned development path for `staticdatagen` toward a stable 1.0 release.

## Current Status: v0.0.6

The library is currently in early development with:
- Core functionality implemented and tested
- 418+ unit tests with 90%+ code coverage
- Comprehensive documentation
- Active CI/CD pipeline

## Version Milestones

### v0.1.0 (Target: Q2 2026)

**Focus: API Stabilization**

- [ ] Audit and finalize public API surface
- [ ] Remove deprecated functions
- [ ] Stabilize error types and messages
- [ ] Document all breaking changes from 0.0.x
- [ ] Add integration test suite

### v0.2.0 (Target: Q3 2026)

**Focus: Performance & Features**

- [ ] Performance benchmarks for all major operations
- [ ] Async support (behind feature flag)
- [ ] Streaming content generation for large sites
- [ ] Memory usage optimization
- [ ] Plugin system design

### v0.3.0 (Target: Q4 2026)

**Focus: Ecosystem Integration**

- [ ] Plugin API stabilization
- [ ] Third-party template engine support
- [ ] Custom output format support
- [ ] Enhanced caching mechanisms

### v1.0.0 (Target: Q1 2027)

**Focus: Production Ready**

- [ ] Semantic versioning commitment
- [ ] Long-term support (LTS) commitment
- [ ] Migration guide from 0.x
- [ ] Production deployment documentation
- [ ] Performance guarantees

## Feature Flags

Current and planned feature flags:

| Flag | Status | Description |
|------|--------|-------------|
| `default` | Stable | Core functionality |
| `async` | Planned | Async/await support |
| `full` | Planned | All optional features |
| `minimal` | Planned | Minimal dependency footprint |
| `serde` | Planned | Serialization support |

## API Stability Guarantees

### Before 1.0
- Breaking changes may occur between minor versions
- Deprecation warnings will be provided when possible
- CHANGELOG will document all breaking changes

### After 1.0
- Semantic versioning strictly followed
- Breaking changes only in major versions
- Minimum 6-month deprecation period
- LTS releases with extended support

## Contributing to the Roadmap

We welcome community input on the roadmap:

1. **Feature Requests**: Open an issue with `[Feature]` prefix
2. **API Feedback**: Comment on existing roadmap issues
3. **RFC Process**: Major changes go through RFC discussion

## Priorities

Current development priorities (in order):

1. **Stability**: Fix bugs and improve reliability
2. **Performance**: Optimize hot paths
3. **Documentation**: Improve examples and guides
4. **Features**: Add requested functionality

## Breaking Change Policy

When breaking changes are necessary:

1. Document in CHANGELOG with migration guide
2. Provide deprecation warnings for at least one release
3. Offer automated migration tools when feasible
4. Maintain compatibility shims where practical

## Timeline Disclaimer

All dates are targets and may be adjusted based on:
- Community feedback and priorities
- Upstream dependency changes
- Resource availability
- Quality requirements

## Questions?

- Open an issue for roadmap discussions
- Join community discussions on GitHub
- Review the [CONTRIBUTING](CONTRIBUTING.md) guide
