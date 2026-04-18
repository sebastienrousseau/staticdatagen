<p align="center">
  <img src="https://cloudcdn.pro/staticdatagen/v1/logos/staticdatagen.svg" alt="StaticDataGen logo" width="128" />
</p>

<h1 align="center">StaticDataGen</h1>

<p align="center">
  <strong>A Rust library for generating structured data files and metadata for static sites.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/staticdatagen/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/staticdatagen/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/staticdatagen"><img src="https://img.shields.io/crates/v/staticdatagen.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/staticdatagen"><img src="https://img.shields.io/badge/docs.rs-staticdatagen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://codecov.io/gh/sebastienrousseau/staticdatagen"><img src="https://img.shields.io/codecov/c/github/sebastienrousseau/staticdatagen?style=for-the-badge&logo=codecov" alt="Coverage" /></a>
  <a href="https://lib.rs/crates/staticdatagen"><img src="https://img.shields.io/badge/lib.rs-v0.0.8-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
</p>

---

## Install

```bash
cargo add staticdatagen
```

Or add to `Cargo.toml`:

```toml
[dependencies]
staticdatagen = "0.0.8"
```

You need [Rust](https://rustup.rs/) 1.88.0 or later. Works on macOS, Linux, and Windows.

---

## Overview

StaticDataGen generates the structured data layer for static sites — HTML pages, RSS feeds, sitemaps, and SEO metadata.

- **HTML generation** from templates and content
- **RSS and Atom feeds** for content syndication
- **XML sitemaps** for search engine discovery
- **SEO meta tags** — Open Graph, Twitter Card, JSON-LD

---

## Features

| | |
| :--- | :--- |
| **HTML generation** | Generate static HTML pages from templates |
| **RSS feeds** | Create RSS and Atom feeds |
| **Sitemaps** | Generate XML sitemaps |
| **SEO meta tags** | Produce Open Graph, Twitter Card, and schema.org metadata |
| **Structured data** | JSON-LD and microdata output |

---

## Usage

```rust,no_run
use staticdatagen::compiler::service::compile;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    compile(
        Path::new("build"),
        Path::new("content"),
        Path::new("public"),
        Path::new("templates"),
    )?;
    println!("Site data generated!");
    Ok(())
}
```

---

## Development

```bash
cargo build        # Build the project
cargo test         # Run all tests
cargo clippy       # Lint with Clippy
cargo fmt          # Format with rustfmt
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, signed commits, and PR guidelines.

---

**THE ARCHITECT** ᴫ [Sebastien Rousseau](https://sebastienrousseau.com)
**THE ENGINE** ᵞ [EUXIS](https://euxis.co) ᴫ Enterprise Unified Execution Intelligence System

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#staticdatagen">Back to Top</a></p>
