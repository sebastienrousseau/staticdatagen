<!-- markdownlint-disable MD033 MD041 -->
<img src="https://kura.pro/staticdatagen/images/logos/staticdatagen.svg"
alt="StaticDataGen logo" height="66" align="right" />
<!-- markdownlint-enable MD033 MD041 -->

# StaticDataGen

A fast, secure, and comprehensive static site structured data generator library written in Rust.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Love][made-with-rust]][08] [![MSRV][msrv-badge]][08] [![Crates.io][crates-badge]][03] [![lib.rs][libs-badge]][01] [![Docs.rs][docs-badge]][04] [![Codecov][codecov-badge]][06] [![Build Status][build-badge]][07] [![GitHub][github-badge]][09]

• [Website][00] • [Documentation][04] • [Report Bug][02] • [Request Feature][02] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Overview 🚀

`staticdatagen` is a robust Rust library that streamlines the generation of structured data and metadata for static sites. It provides a comprehensive suite of tools for creating HTML files, RSS feeds, sitemaps, and SEO-friendly metadata, with built-in security features and multi-language support.

## Features ✨

### Content Generation & Processing

- **Markdown to HTML Conversion**
  - Clean, semantic HTML output
  - Syntax highlighting support
  - Custom template integration
  - Content minification

- **Metadata Generation**
  - OpenGraph and Twitter Card meta tags
  - JSON-LD support for rich snippets
  - Automatic metadata extraction
  - SEO optimization

### Site Structure & Navigation

- **Navigation System**
  - Automatic menu generation
  - Hierarchical structure support
  - Accessibility-compliant markup
  - Customizable formatting

- **Multi-language Support**
  - Built-in support for English, French, and German
  - Extensible translation system
  - Language-specific templates
  - i18n content management

### Data Format Support

- **Sitemaps**
  - Standard XML sitemaps
  - Google News sitemaps
  - Automatic URL generation
  - Change frequency tracking

- **Feed Generation**
  - RSS 2.0 support
  - Atom feed generation
  - Custom feed templates
  - Auto-updating timestamps

### Security & Validation

- **Security Features**
  - Path traversal prevention
  - Input sanitization
  - URL validation
  - Security.txt generation (RFC 9116)

- **Data Validation**
  - Metadata verification
  - URL structure checking
  - Language code validation
  - Content integrity checks

### Developer Experience

- **Error Handling**
  - Comprehensive error types
  - Detailed error messages
  - Context-aware failures
  - Recovery suggestions

- **Performance**
  - Efficient file processing
  - HTML minification
  - Parallel processing where possible
  - Memory-efficient operations

## Installation 📦

Add `staticdatagen` to your `Cargo.toml`:

```toml
[dependencies]
staticdatagen = "0.0.2"
```

## Directory Structure 📁

Create the following directory structure for your project:

```bash
your-project/
├── content/         # Your Markdown content
├── templates/       # HTML templates
├── build/          # Temporary build directory
└── site/           # Final output directory
```

## Usage 💻

### Basic Example

```rust
use staticdatagen::compiler::service::compile;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define paths for your static site
    let build_dir = Path::new("examples/build");            // Temporary build directory
    let content_dir = Path::new("examples/content");        // Source content directory
    let site_dir = Path::new("examples/site");              // Output directory
    let template_dir = Path::new("examples/templates");     // HTML templates directory

    // Compile the static site
    compile(
        build_dir,
        content_dir,
        site_dir,
        template_dir,
    )?;

    Ok(())
}
```

## Generated Files 📄

The library generates the following files:

- **HTML Files**
  - Minified and optimized
  - Accessibility-compliant
  - SEO-friendly structure

- **Metadata Files**
  - `security.txt` - Security policy (RFC 9116)
  - `humans.txt` - Site credits and information
  - `robots.txt` - Crawler instructions
  - `manifest.json` - PWA manifest

- **SEO Files**
  - `sitemap.xml` - Standard sitemap
  - `news-sitemap.xml` - Google News sitemap
  - `rss.xml` - Content syndication feed

## Error Handling 🛠️

The library uses the `anyhow` crate for error handling, providing detailed error messages and context. All public functions return `Result` types with comprehensive error information:

```rust
use anyhow::Result;

fn main() -> Result<()> {
    // Your code here
    Ok(())
}
```

## Performance 🚀

- **File Processing**: Efficient streaming for large files
- **Minification**: Optimized HTML output
- **Caching**: Template and content caching
- **Memory Usage**: Minimal memory footprint

## Documentation 📚

For full API documentation, please visit [docs.rs/staticdatagen][04].

## Examples 🎯

To explore more examples, clone the repository and run the following command:

```shell
cargo run --example example_name
```

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request.

## License 📜

This project is licensed under either of

- [Apache License, Version 2.0][10]
- [MIT license][11]

at your option.

## Acknowledgements 💝

Special thanks to all contributors who have helped build the `staticdatagen` library.

[00]: https://staticdatagen.com
[01]: https://lib.rs/crates/staticdatagen
[02]: https://github.com/sebastienrousseau/staticdatagen/issues
[03]: https://crates.io/crates/staticdatagen
[04]: https://docs.rs/staticdatagen
[05]: https://github.com/sebastienrousseau/staticdatagen/blob/main/CONTRIBUTING.md
[06]: https://codecov.io/gh/sebastienrousseau/staticdatagen
[07]: https://github.com/sebastienrousseau/staticdatagen/actions?query=branch%3Amain
[08]: https://www.rust-lang.org/
[09]: https://github.com/sebastienrousseau/staticdatagen
[10]: https://www.apache.org/licenses/LICENSE-2.0
[11]: https://opensource.org/licenses/MIT

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/staticdatagen/release.yml?branch=main&style=for-the-badge&logo=github
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/staticdatagen?style=for-the-badge&token=yGOBUANPm3&logo=codecov
[crates-badge]: https://img.shields.io/crates/v/staticdatagen.svg?style=for-the-badge&color=fc8d62&logo=rust
[docs-badge]: https://img.shields.io/badge/docs.rs-staticdatagen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/staticdatagen-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.2-orange.svg?style=for-the-badge
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust
[msrv-badge]: https://img.shields.io/badge/MSRV-1.56.0-blue.svg?style=for-the-badge
