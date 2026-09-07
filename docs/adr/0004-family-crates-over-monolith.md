# 0004. Build on the family's single-purpose crates

- **Status:** accepted
- **Date:** 2024-10-27 (recorded 2026-09-07)

## Context

Front-matter parsing, metadata processing, Markdown rendering,
templating, translation and sitemap construction are all needed here.
Each could live inside this crate, which would mean one repository, one
release and no version skew.

## Decision

Each is a separate crate — `frontmatter-gen`, `metadata-gen`,
`html-generator`, `staticweaver`, `langweave`, `sitemap-gen` — and
staticdatagen composes them.

## Consequences

- Each piece is usable on its own, tested on its own, and fuzzed on its
  own. A bug in front-matter parsing is fixed once, for every consumer.
- The cost is real: a change that spans two crates needs two releases in
  dependency order, and a bump here is a deliberate release with a
  changelog entry rather than a silent upgrade. Exact-version pins and
  `cargo-vet` publisher trust make that explicit rather than accidental.
- The alternative — vendoring the code — was rejected because it hides
  which version of a parser produced a given site.
