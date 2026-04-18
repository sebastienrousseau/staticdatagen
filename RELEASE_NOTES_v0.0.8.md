# Release: v0.0.8

## Highlights
- **Dependency Refresh** — Updated all core dependencies including `rlg`, `comrak`, `pulldown-cmark`, `http-handle`, and `langweave`
- **Shared Utilities** — Integrated the new `euxis-commons` local dependency for future-proof shared components
- **Logging Modernization** — Refactored internal logging macros to use the latest `rlg` 0.0.8 builder API and non-blocking flusher
- **Workflow Hardening** — Updated GitHub Actions workflows to use the latest `upload-artifact` (v7) and `download-artifact` (v8) versions

## Dependency Updates
- **rlg** — Updated from 0.0.6 to 0.0.8 (Logging engine refresh)
- **comrak** — Updated from 0.50 to 0.51 (Markdown parsing)
- **pulldown-cmark** — Updated from 0.12 to 0.13 (CommonMark compliance)
- **http-handle** — Updated from 0.0.2 to 0.0.4 (Networking utilities)
- **langweave** — Updated from 0.0.1 to 0.0.2 (I18n support)
- **euxis-commons** — Added as a local dependency (Shared utilities)

## CI/CD Improvements
- **GitHub Actions** — Updated `actions/upload-artifact` to v7 and `actions/download-artifact` to v8 across all workflows
- **Workflow Reliability** — Improved `release` and `strict-ci` pipeline robustness with latest action versions

## Reliability Enhancements
- **Logging** — Refactored `macro_log_info!` to align with the new `rlg` 0.0.8 builder pattern and fire-and-forget ingestion
- **Type Safety** — Hardened session ID handling in logging macros with explicit type conversions

## Breaking Changes
None. This release maintains full backward compatibility for users.

## Upgrade Instructions
1. Update your `Cargo.toml` dependency: `staticdatagen = "0.0.8"`
2. Run `cargo update` to fetch the latest version
3. No code changes required — the updated logging macros are internal to the crate

## Migration Guide
No migration required. All existing code will continue to work without modification.

---

*Engineered with [Euxis](https://euxis.co/) — Enterprise Unified eXecution Intelligence System*
