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

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/staticdatagen/release.yml?branch=main&style=for-the-badge&logo=github
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/staticdatagen?style=for-the-badge&token=yGOBUANPm3&logo=codecov
[crates-badge]: https://img.shields.io/crates/v/staticdatagen.svg?style=for-the-badge&color=fc8d62&logo=rust
[docs-badge]: https://img.shields.io/badge/docs.rs-staticdatagen-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/staticdatagen-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.5-orange.svg?style=for-the-badge
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust
[msrv-badge]: https://img.shields.io/badge/MSRV-1.58.0-blue.svg?style=for-the-badge

## Changelog 📚
