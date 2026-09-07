# `staticdatagen` architecture

How the crate is put together, for contributors. The user-facing story
is in the [README](../README.md); this page is about the shape of the
code and the decisions behind it.

## Layout

```text
staticdatagen/
├── src/
│   ├── lib.rs          # public surface and re-exports
│   ├── compiler/       # the build: read content, render, write a site
│   ├── generators/     # per-artefact writers: sitemap, manifest, humans, tags, CNAME
│   ├── models/         # the data the pipeline passes around
│   ├── modules/        # content transforms: preprocess, post-process, plain text, JSON, navigation
│   ├── locales/        # translated strings
│   ├── macros/         # declarative helpers used across the crate
│   └── utilities/      # files, directories, security, backups, elements
├── tests/              # integration suites over a temporary site
├── examples/           # 25 runnable examples
├── benches/            # Criterion: throughput, scaling, stress
├── fuzz/               # libFuzzer targets, seed corpus, regression inputs
├── docs/adr/           # architecture decision records
└── supply-chain/       # cargo-vet state
```

## The pipeline

`compiler::service::compile` is the whole crate in one call. It takes a
content directory and produces a finished site:

1. **Walk the content directory** with `WalkDir::sort_by_file_name`.
   The sort is not cosmetic: without it, entry order follows the
   filesystem, APFS and ext4 disagree, and two builds of the same
   content differ. See [ADR-0002](adr/0002-sorted-directory-walk.md).
2. **Read front matter and body** for each file, through
   `frontmatter-gen`.
3. **Preprocess** the content: class annotations and image attributes
   are rewritten into the shapes the renderer expects.
4. **Render** through `html-generator`, which handles Markdown,
   accessibility and SEO metadata.
5. **Post-process** the generated HTML.
6. **Generate the site's other artefacts**: `sitemap.xml`,
   `news_sitemap.xml`, `manifest.json`, `humans.txt`, `CNAME`, tag
   pages, plain-text and JSON views.
7. **Write everything** through `utilities::security::sanitize_path`,
   which is the only way a path reaches the filesystem.

## Where the crate sits in the family

staticdatagen is the layer that assembles a site out of the family's
single-purpose crates:

| Crate | Provides |
|---|---|
| `frontmatter-gen` | front-matter extraction and parsing |
| `metadata-gen` | metadata processing and `<meta>` tag generation |
| `html-generator` | Markdown to accessible, SEO-ready HTML |
| `staticweaver` | the template engine |
| `langweave` | translated strings |
| `sitemap-gen` | sitemap construction |

Each is pinned in `Cargo.toml` and trusted by publisher in
`cargo-vet`. A bump to any of them is a deliberate release here, with a
changelog entry, rather than a silent upgrade.

## Determinism

Two builds of the same content must produce byte-identical output, on
any filesystem and any platform. That is what makes a golden-file
comparison in a downstream project meaningful, and it is why the
directory walk sorts and why tag and listing order is total rather than
insertion-ordered.

## Path safety

`utilities::security::sanitize_path` rejects empty paths, `..`
traversal, and paths that escape the directory they resolve against;
`validate_directory` checks a directory is usable for its stated purpose
before anything is written. Every write in the compiler goes through
them. `fuzz_path` pins two properties: an accepted path never contains
a `..` component, and sanitising an accepted path again returns it
unchanged.

## Tests

- `src/**` `#[cfg(test)]` — unit tests next to the code.
- `tests/integration_tests.rs` and `tests/compiler_integration.rs` —
  the compiler end to end over a temporary directory.
- `fuzz/` — three targets over the string-walking transforms and the
  path boundary.

## Coverage

The gate is 98% lines with no exclusions: the crate is a library with no
binary entry point and no target-gated glue, so every line is reachable
from a test. See [`DEVELOPMENT.md`](../DEVELOPMENT.md).

## Where to read next

- [`docs/adr/`](adr/README.md) for the decisions that shape the above.
- [`DEVELOPMENT.md`](../DEVELOPMENT.md) for reproducing every CI gate.
