<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/staticdatagen/v1/logos/staticdatagen.svg" alt="StaticDataGen logo" width="128" />
</p>

<h1 align="center">staticdatagen</h1>

<p align="center">
  The structured-data layer for Rust static sites — HTML, RSS, sitemaps,
  Open Graph, Twitter Card, and JSON-LD from one content tree, with zero
  <code>unsafe</code> and the YAML/markdown stack pre-vetted.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/staticdatagen/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/staticdatagen/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/staticdatagen"><img src="https://img.shields.io/crates/v/staticdatagen.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/staticdatagen"><img src="https://img.shields.io/badge/docs.rs-staticdatagen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://codecov.io/gh/sebastienrousseau/staticdatagen"><img src="https://img.shields.io/codecov/c/github/sebastienrousseau/staticdatagen?style=for-the-badge&logo=codecov" alt="Coverage" /></a>
  <a href="https://lib.rs/crates/staticdatagen"><img src="https://img.shields.io/badge/lib.rs-v0.0.10-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install) — Cargo, MSRV, features
- [Quick Start](#quick-start) — compile a site in ten lines

**Library reference**

- [Why staticdatagen?](#why-staticdatagen) — design rationale
- [Capabilities in 0.0.10](#capabilities-in-0010) — release inventory
- [Modules](#modules) — top-level surface
- [Library Usage](#library-usage) — `compile`, errors, UUID, version
- [Configuration](#configuration) — what gets emitted
- [Examples](#examples) — runnable example index

**Operational**

- [When not to use staticdatagen](#when-not-to-use-staticdatagen) — limitations
- [Roadmap](#roadmap) — v0.0.11 → v0.0.15
- [Development](#development) — local loop, CI
- [Security](#security) — guarantees and audit cadence
- [Documentation](#documentation) — reference links
- [License](#license)

---

## Install

```toml
[dependencies]
staticdatagen = "0.0.10"
```

Or via Cargo:

```bash
cargo add staticdatagen
```

### MSRV

| Crate | MSRV | Notes |
|---|---|---|
| `staticdatagen` | **1.88.0** | Pinned via `rust-version` in `Cargo.toml`; enforced by the dedicated MSRV CI job. |

Tested on macOS (Intel + Apple Silicon), Linux (x86_64 GNU + musl), and Windows (x86_64-msvc).

### Cargo features

| Feature | Default? | Pulls in | Adds | Status |
|---|:---:|---|---|---|
| `full` | ✅ | `rss + sitemap + i18n + server` | Convenience meta-feature (the default). | Stable |
| `rss` | ✅ (via `full`) | — | RSS feed emission knob. | Currently always-on; per-feature gating tracked in [#78](https://github.com/sebastienrousseau/staticdatagen/issues/78) for v0.0.11. |
| `sitemap` | ✅ (via `full`) | — | Sitemap + news-sitemap emission knob. | Currently always-on; per-feature gating tracked in [#78](https://github.com/sebastienrousseau/staticdatagen/issues/78) for v0.0.11. |
| `i18n` | ✅ (via `full`) | `langweave 0.0.2` | Enables the `locales` module for translated string handling. | Active. |
| `server` | ✅ (via `full`) | `http-handle 0.0.5` | Re-exports `staticdatagen::Server` for serving a built site. | Active. **AGPL transitive — exits in v0.0.12 ([#83](https://github.com/sebastienrousseau/staticdatagen/issues/83))**. |
| `minimal` | ❌ | — | Reserved for a smaller surface; currently equivalent to disabling `full`. Real gating in v0.0.11 ([#78](https://github.com/sebastienrousseau/staticdatagen/issues/78)). |
| `async` | ❌ | — | Reserved name; no async surface yet. |
| `serde` | ❌ | — | Always-on via the unconditional `serde` direct dep. Reserved flag. |

```toml
# Example: smaller binary, no preview server (no AGPL transitive)
[dependencies]
staticdatagen = { version = "0.0.10", default-features = false, features = ["i18n"] }
```

---

## Quick Start

Compile a content tree into a publishable static site. The example
below is doctest-runnable: every type, function, and import resolves
in `staticdatagen 0.0.10`.

```rust,no_run
use staticdatagen::compile;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Inputs:
    //   - `content/`   markdown files with YAML frontmatter
    //   - `templates/` HTML templates (staticweaver / MiniJinja syntax)
    // Outputs (after the build directory is renamed to the site path):
    //   - `public/index.html`, `public/sitemap.xml`, `public/rss.xml`,
    //     `public/manifest.json`, `public/robots.txt`, `public/humans.txt`,
    //     `public/security.txt`, `public/CNAME`, `public/<page>/index.html`.
    compile(
        Path::new("build"),      // build_dir_path  — scratch space
        Path::new("content"),    // content_path    — markdown sources
        Path::new("public"),     // site_path       — final output root
        Path::new("templates"),  // template_path   — HTML templates
    )?;
    Ok(())
}
```

Pre-flight checklist for `content/`:

- Each `.md` file carries YAML frontmatter (`---` delimited).
- Frontmatter sets at least `title`, `description`, `permalink`.
- Optional but recommended: `layout` (defaults to `"page"`),
  `author`, `changefreq`, `last_modified`, `og:image`, `twitter:image`.
- Subdirectories are walked recursively; per-locale trees
  (`content/fr/`, `content/_posts/ar/`) are preserved into the
  output URL.

---

## Why staticdatagen?

The crate covers the *structured-data layer* of a static site — the
parts a search engine, an RSS reader, an LLM crawler, or a manifest
consumer parses, as distinct from the HTML body itself.

Three design choices motivate the rewrite from a hand-rolled
sitemap-plus-RSS pipeline:

1. **One content tree, every artefact.** Frontmatter is parsed once
   per page (via `metadata-gen`), the rendered HTML is emitted by
   `comrak` + `staticweaver`, and the same metadata feeds RSS,
   sitemap, JSON-LD, Open Graph, Twitter Card, manifest, robots,
   humans, security, and CNAME emitters. No duplication, no drift
   between the HTML you publish and the metadata that describes it.

2. **`#![forbid(unsafe_code)]` at the crate root.** Every direct
   dependency is pure-Rust; the YAML stack migrated to `noyalib`
   (see [v0.0.9 CHANGELOG](CHANGELOG.md)) which drops the C-FFI
   `libyml` link entirely. `cargo audit`, `cargo deny`, and `miri`
   gate every push.

3. **Honest defaults.** Missing `layout` resolves to `"page"`,
   missing `tags/index.html` template no-ops the tag write, missing
   `main.js` / `sw.js` are debug-logged and skipped instead of
   aborting the build. The build succeeds for the realistic case
   where your content tree is mid-evolution.

A few features built on top:

- **Multilingual trees.** `content/<lang>/_posts/...` walks
  recursively (`walkdir`), the locale prefix is preserved through
  filename mangling (`with_extension("")`), and the `i18n` feature
  enables `langweave`-backed translation lookups.
- **Build-state log fidelity.** The "Successfully generated…" log
  line fires only *after* the build has truly succeeded — log
  scrapers (CI, `ssg`) can rely on it as a build-state signal.
- **Parallel-ready data model.** `FileData` and `PageData` are
  `Send + Sync`. The compile pipeline parallelises across cores in
  v0.0.11 ([#74](https://github.com/sebastienrousseau/staticdatagen/issues/74)).

---

## Capabilities in 0.0.10

The 0.0.10 release is a stabilisation pass on the v0.0.9 dep-graph
slim-down plus five user-reported bug fixes. See
[`CHANGELOG.md`](CHANGELOG.md) for the full entry.

| Theme | Headline deliverables |
| :--- | :--- |
| Bug fixes | Empty `layout:` resolves to `"page"` ([#67]); missing `main.js` / `sw.js` no-op ([#68]); missing tags template no-op ([#69]); recursive walk through subdirectories with locale-preservation ([#70]); success log line fires only after success ([#71]). |
| Security | `rand` 0.8.5 → 0.8.6 closes GHSA-cq8v-f236-94qc / RUSTSEC-2026-0097 (Stacked-Borrows unsoundness reachable through custom loggers). |
| Dependencies | `staticweaver` 0.0.2 → 0.0.3, `rss-gen` 0.0.5 → 0.0.6, `regex` 1.11 → 1.12, `walkdir` 2 → 2.5, `uuid` 1.11 → 1.23, `idna` 1.0 → 1.1, `rayon` 1.10 → 1.12. Dev: `proptest` 1.6 → 1.11, `tempfile` 3 → 3.27. |
| Tests | 714 lib tests, 55 doctests, integration suite green; 0 `cargo audit` vulnerabilities. |

[#67]: https://github.com/sebastienrousseau/staticdatagen/issues/67
[#68]: https://github.com/sebastienrousseau/staticdatagen/issues/68
[#69]: https://github.com/sebastienrousseau/staticdatagen/issues/69
[#70]: https://github.com/sebastienrousseau/staticdatagen/issues/70
[#71]: https://github.com/sebastienrousseau/staticdatagen/issues/71

---

## Modules

Top-level public modules, in pipeline order.

| Module | Purpose |
| :--- | :--- |
| `compiler` | The orchestrator. `compiler::service::compile` walks `content/`, processes each file, and writes the site. |
| `generators` | Per-artefact emitters: `cname`, `humans`, `manifest`, `news_sitemap`, `tags`. Each is independently callable. |
| `models` | Core data types: `FileData`, `PageData`, `CnameData`, `SecurityData`, manifest + validation helpers. |
| `modules` | Cross-cutting passes: `navigation`, `plaintext`, `postprocessor`, `preprocessor`, `robots`, `security`, JSON-LD output. |
| `utilities` | Reusable helpers: `backup`, `directory`, `element`, `file`, `security` (path sanitization), `uuid`, `write`. |
| `locales` (`i18n` feature) | Translation lookups via `langweave`. |
| `macros` | Boilerplate-reduction macros: `macro_create_directories!`, `macro_cleanup_directories!`, `macro_metadata_option!`, `macro_log_info!`. |

---

## Library Usage

<details>
<summary><b>Compile a site (the common case)</b></summary>

```rust,no_run
use staticdatagen::compile;
use std::path::Path;

fn build() -> anyhow::Result<()> {
    compile(
        Path::new("build"),
        Path::new("content"),
        Path::new("public"),
        Path::new("templates"),
    )?;
    Ok(())
}
```

`compile()` is idempotent if `content/` and `templates/` are
unchanged — the final `fs::rename(build_dir → site_path)` is atomic
on POSIX. The function returns `anyhow::Result<()>` in v0.0.10;
v0.0.11 ([#73]) promotes the typed
[`staticdatagen::Error`](https://docs.rs/staticdatagen/latest/staticdatagen/enum.Error.html)
to the public surface.

[#73]: https://github.com/sebastienrousseau/staticdatagen/issues/73

</details>

<details>
<summary><b>Error handling</b></summary>

The internal `Error` enum is exported; today `compile()` returns
`anyhow::Result<()>` so downcasting is the access path.

```rust,no_run
use staticdatagen::{compile, Error};
use std::path::Path;

fn main() {
    let result = compile(
        Path::new("build"),
        Path::new("content"),
        Path::new("public"),
        Path::new("templates"),
    );

    match result {
        Ok(()) => println!("Site built."),
        Err(e) => match e.downcast_ref::<Error>() {
            Some(Error::Io { context, .. }) => {
                eprintln!("I/O failed: {context}");
            }
            Some(Error::Template(msg)) => {
                eprintln!("Template error: {msg}");
            }
            Some(Error::ContentProcessing { message, .. }) => {
                eprintln!("Content error: {message}");
            }
            Some(Error::Config(msg)) => eprintln!("Config error: {msg}"),
            Some(Error::Other(msg)) => eprintln!("Error: {msg}"),
            None => eprintln!("Wrapped error: {e:#}"),
        },
    }
}
```

</details>

<details>
<summary><b>Build a typed error</b></summary>

```rust
use staticdatagen::{Error, ErrorSeverity};

let err = Error::content_processing_builder()
    .message("Invalid frontmatter")
    .context("expected key `title`")
    .severity(ErrorSeverity::Error)
    .build();

assert!(matches!(err, Error::ContentProcessing { .. }));
```

The matching `IoErrorBuilder` mirrors the pattern for
[`Error::Io`](https://docs.rs/staticdatagen/latest/staticdatagen/enum.Error.html#variant.Io)
with `.source`, `.operation`, `.path`, and `.context` setters.

</details>

<details>
<summary><b>Unique identifiers</b></summary>

```rust
use staticdatagen::generate_unique_string;

let id_a = generate_unique_string();
let id_b = generate_unique_string();

// UUID v4: 36 characters with hyphens.
assert_eq!(id_a.len(), 36);
assert_ne!(id_a, id_b);
```

</details>

<details>
<summary><b>Version constant</b></summary>

```rust
// VERSION is sourced from CARGO_PKG_VERSION at build time.
println!("staticdatagen {}", staticdatagen::VERSION);
```

</details>

<details>
<summary><b>Serve the built site (<code>server</code> feature)</b></summary>

```rust,ignore
# // Requires the `server` feature.
use staticdatagen::Server;
use std::path::Path;

// Re-exported from `http_handle` — see that crate's docs for the
// full configuration surface. AGPL transitive in v0.0.10; replaced
// by `axum` in v0.0.12 (#83).
let _server = Server::new("127.0.0.1:3000", Path::new("public"));
```

</details>

---

## Configuration

`compile()` takes four paths; everything else is driven by per-page
YAML frontmatter. The recognised keys (parsed by `metadata-gen`)
include:

| Key | Type | Purpose |
| :--- | :--- | :--- |
| `title` | string | Page `<title>`, Open Graph, RSS item title. |
| `description` | string | Meta description, RSS item, JSON-LD. |
| `permalink` | URL | Canonical URL, used by sitemap, RSS, manifest. |
| `layout` | string | Template name (without `.html`). Defaults to `"page"` when absent. |
| `author` | string | RSS `<author>`, JSON-LD `author`. |
| `changefreq` | string | sitemap `<changefreq>`. |
| `last_modified` | RFC 3339 / RFC 2822 date | sitemap `<lastmod>`, news-sitemap. |
| `tags` | comma-separated | Tag aggregation into `tags/index.html`. |
| `og:image`, `twitter:image` | URL | Social cards. |
| `cname` | hostname | Written to `CNAME`. |
| `news_genres`, `news_keywords`, `news_publication_date` | strings | News sitemap fields. |

Files emitted to the site root: `index.html`, `manifest.json`,
`robots.txt`, `humans.txt`, `security.txt`, `sitemap.xml`,
`news-sitemap.xml`, `rss.xml`, `CNAME`. Non-index pages land at
`<page>/index.html`.

---

## Examples

25 runnable examples ship under `examples/`. Each one is built and
linted in CI.

| Surface | Example | Run with |
| :--- | :--- | :--- |
| End-to-end | `service_example.rs` | `cargo run --example service_example` |
| End-to-end | `compiler_example.rs` | `cargo run --example compiler_example` |
| End-to-end | `static_site_example.rs` | `cargo run --example static_site_example` |
| End-to-end | `lib_example.rs` | `cargo run --example lib_example` |
| Generator | `cname_example.rs` | `cargo run --example cname_example` |
| Generator | `humans_example.rs` | `cargo run --example humans_example` |
| Generator | `manifest_example.rs` | `cargo run --example manifest_example` |
| Generator | `news_sitemap_example.rs` | `cargo run --example news_sitemap_example` |
| Generator | `tags_example.rs` | `cargo run --example tags_example` |
| Generator | `security_example.rs` | `cargo run --example security_example` |
| Generator | `robots_example.rs` | `cargo run --example robots_example` |
| Module | `navigation_example.rs` | `cargo run --example navigation_example` |
| Module | `plaintext_example.rs` | `cargo run --example plaintext_example` |
| Module | `postprocessor_example.rs` | `cargo run --example postprocessor_example` |
| Module | `preprocessor_example.rs` | `cargo run --example preprocessor_example` |
| Module | `json_example.rs` | `cargo run --example json_example` |
| Models | `data_example.rs` | `cargo run --example data_example` |
| Utilities | `backup_example.rs` | `cargo run --example backup_example` |
| Utilities | `directory_example.rs` | `cargo run --example directory_example` |
| Utilities | `element_example.rs` | `cargo run --example element_example` |
| Utilities | `file_example.rs` | `cargo run --example file_example` |
| Utilities | `uuid_example.rs` | `cargo run --example uuid_example` |
| Utilities | `write_example.rs` | `cargo run --example write_example` |
| i18n | `locales_de_example.rs` | `cargo run --example locales_de_example --features i18n` |
| i18n | `locales_fr_example.rs` | `cargo run --example locales_fr_example --features i18n` |

A narrative *learn → integrate → extend* ladder lands in v0.0.15
([#98](https://github.com/sebastienrousseau/staticdatagen/issues/98)).

---

## When not to use staticdatagen

- **You want a full SSG, not the structured-data layer.** Reach for
  [Zola](https://www.getzola.org/), [Cobalt](https://cobalt-org.github.io/),
  or [Hugo](https://gohugo.io/). `staticdatagen` is the library that
  emits the parts a search engine, RSS reader, or LLM crawler reads —
  it does not ship a CLI, theme system, or asset pipeline.
- **You need a non-Rust runtime today.** WASI 0.2 Component Model
  distribution lands in v0.0.14
  ([#90](https://github.com/sebastienrousseau/staticdatagen/issues/90));
  Python / Node FFIs in v0.0.15
  ([#96](https://github.com/sebastienrousseau/staticdatagen/issues/96),
  [#97](https://github.com/sebastienrousseau/staticdatagen/issues/97)).
- **You need true incremental rebuilds.** v0.0.10 rebuilds every
  file. Content-hash-based incremental cache lands in v0.0.13
  ([#87](https://github.com/sebastienrousseau/staticdatagen/issues/87),
  closes long-standing
  [#36](https://github.com/sebastienrousseau/staticdatagen/issues/36)).

---

## Roadmap

| Milestone | Theme | Highlights |
| :--- | :--- | :--- |
| [v0.0.11](https://github.com/sebastienrousseau/staticdatagen/milestone/1) | Tier-1 quick wins | Typed `Error` ([#73]), parallel pipeline ([#74]), drop `pulldown-cmark` ([#75]), `cargo-vet` ([#76]), SBOM ([#77]), real feature gating ([#78]). |
| [v0.0.12](https://github.com/sebastienrousseau/staticdatagen/milestone/2) | DX & observability | `CompileBuilder` ([#79]), `tracing` migration ([#80]), `cargo-fuzz` ([#81]), Loom ([#82]), AGPL exit / `axum` ([#83]), god-object split ([#84]). |
| [v0.0.13](https://github.com/sebastienrousseau/staticdatagen/milestone/3) | AI-discoverable static sites | `llms.txt` ([#85]), AI sitemap ([#86]), incremental cache ([#87], closes [#36]), ADRs ([#88]), public Criterion charts ([#89]). |
| [v0.0.14](https://github.com/sebastienrousseau/staticdatagen/milestone/4) | Portability | WASI 0.2 component ([#90]), `no_std + alloc` core split ([#91]), `Reporter` trait ([#92]), architecture diagram ([#93]). |
| [v0.0.15](https://github.com/sebastienrousseau/staticdatagen/milestone/5) | Correctness & GTM | Kani proofs ([#94]), differential fuzz ([#95]), PyO3 binding ([#96]), NAPI binding ([#97]), README + examples ladder ([#98]). |

[#36]: https://github.com/sebastienrousseau/staticdatagen/issues/36
[#73]: https://github.com/sebastienrousseau/staticdatagen/issues/73
[#74]: https://github.com/sebastienrousseau/staticdatagen/issues/74
[#75]: https://github.com/sebastienrousseau/staticdatagen/issues/75
[#76]: https://github.com/sebastienrousseau/staticdatagen/issues/76
[#77]: https://github.com/sebastienrousseau/staticdatagen/issues/77
[#78]: https://github.com/sebastienrousseau/staticdatagen/issues/78
[#79]: https://github.com/sebastienrousseau/staticdatagen/issues/79
[#80]: https://github.com/sebastienrousseau/staticdatagen/issues/80
[#81]: https://github.com/sebastienrousseau/staticdatagen/issues/81
[#82]: https://github.com/sebastienrousseau/staticdatagen/issues/82
[#83]: https://github.com/sebastienrousseau/staticdatagen/issues/83
[#84]: https://github.com/sebastienrousseau/staticdatagen/issues/84
[#85]: https://github.com/sebastienrousseau/staticdatagen/issues/85
[#86]: https://github.com/sebastienrousseau/staticdatagen/issues/86
[#87]: https://github.com/sebastienrousseau/staticdatagen/issues/87
[#88]: https://github.com/sebastienrousseau/staticdatagen/issues/88
[#89]: https://github.com/sebastienrousseau/staticdatagen/issues/89
[#90]: https://github.com/sebastienrousseau/staticdatagen/issues/90
[#91]: https://github.com/sebastienrousseau/staticdatagen/issues/91
[#92]: https://github.com/sebastienrousseau/staticdatagen/issues/92
[#93]: https://github.com/sebastienrousseau/staticdatagen/issues/93
[#94]: https://github.com/sebastienrousseau/staticdatagen/issues/94
[#95]: https://github.com/sebastienrousseau/staticdatagen/issues/95
[#96]: https://github.com/sebastienrousseau/staticdatagen/issues/96
[#97]: https://github.com/sebastienrousseau/staticdatagen/issues/97
[#98]: https://github.com/sebastienrousseau/staticdatagen/issues/98

---

## Development

```bash
git clone https://github.com/sebastienrousseau/staticdatagen.git
cd staticdatagen
make                  # fmt + clippy + test (see Makefile)
cargo build           # debug build
cargo test            # 714 lib + 55 doc tests + integration suite
cargo clippy          # -D warnings is enforced
cargo bench           # Criterion benches under benches/
```

CI runs on every push:

- **Build matrix**: stable, beta, nightly, MSRV (1.88.0) on Linux,
  macOS, and Windows.
- **Miri**: `cargo miri test models -- --skip proptest`,
  `cargo miri test utilities::element`, `utilities::uuid`, `locales`,
  `macros::directory`, `macros::custom`.
- **Coverage**: `cargo llvm-cov` reported to Codecov.
- **Supply chain**: `cargo audit`, `cargo deny`. `cargo vet` lands
  in v0.0.11
  ([#76](https://github.com/sebastienrousseau/staticdatagen/issues/76)).

See [CONTRIBUTING.md](CONTRIBUTING.md) for signed-commit setup and
PR guidelines.

---

## Security

- **`#![forbid(unsafe_code)]`** at the crate root. Verified on every
  push.
- **`#![deny(clippy::unwrap_used, clippy::expect_used)]`** for
  non-test code. Library code paths cannot panic on `unwrap`.
- **Property tests** under `proptest-regressions/` cover the path
  sanitization surface (`utilities::security`).
- **Audit cadence.** `deny.toml` records a quarterly review schedule
  (next: 2026-07-17) and lists every transitive exemption with a
  justification.
- **Vulnerability disclosure.** See [SECURITY.md](.github/SECURITY.md).
- **SBOM** publication lands in v0.0.11
  ([#77](https://github.com/sebastienrousseau/staticdatagen/issues/77));
  Kani proofs in v0.0.15
  ([#94](https://github.com/sebastienrousseau/staticdatagen/issues/94)).

---

## Documentation

- API reference: <https://docs.rs/staticdatagen>
- Crate page: <https://crates.io/crates/staticdatagen>
- Project site: <https://staticdatagen.com>
- Changelog: [`CHANGELOG.md`](CHANGELOG.md)
- Contributing: [`CONTRIBUTING.md`](CONTRIBUTING.md)
- Security policy: [`.github/SECURITY.md`](.github/SECURITY.md)
- Authors: [`AUTHORS.md`](AUTHORS.md)

---

**THE ARCHITECT** ᴫ [Sebastien Rousseau](https://sebastienrousseau.com)
**THE ENGINE** ᵞ [EUXIS](https://euxis.co) ᴫ Enterprise Unified Execution Intelligence System

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#staticdatagen">Back to Top</a></p>
