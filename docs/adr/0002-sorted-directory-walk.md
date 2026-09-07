# 0002. The content walk sorts, so builds are reproducible

- **Status:** accepted
- **Date:** 2026-09-05

## Context

`utilities::file::add` walked the content directory with `WalkDir` and
took entries in whatever order the filesystem returned them. APFS and
ext4 do not agree on that order, so two builds of the same content
produced different output on different machines: tag pages listed their
members in a different sequence, and anything comparing generated files
byte for byte failed on one platform and passed on the other.

The failure was invisible locally. A determinism check that builds twice
on one runner compares two builds on one filesystem, which always agree.
It surfaced in a downstream project's golden-file suite, seeded on macOS
and run on Linux.

## Decision

`WalkDir::new(path).sort_by_file_name()`. Every directory's entries are
ordered by name before they are visited.

## Consequences

- Two builds of the same content produce byte-identical output on any
  filesystem. That is what makes a downstream golden-file comparison
  meaningful.
- Ordering elsewhere in the pipeline must be total for the same reason:
  a tie broken by insertion order reintroduces the problem one layer up.
- The sort costs one comparison per sibling. At the scale a static site
  reaches, that is not measurable.
