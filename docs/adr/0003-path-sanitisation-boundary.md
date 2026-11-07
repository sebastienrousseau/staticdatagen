# 0003. Every filesystem write goes through one sanitiser

- **Status:** accepted
- **Date:** 2026-06-28 (recorded 2026-09-07)

## Context

The compiler writes many kinds of artefact — pages, sitemaps, manifests,
tag indexes — from several modules. Each could construct its own output
path from a title, a slug or a front-matter field, all of which an
author controls. Scattering that logic means one forgotten check is a
path traversal.

## Decision

`utilities::security::sanitize_path` is the single boundary. It rejects
empty paths, `..` traversal, and paths that escape the directory they
resolve against; `validate_directory` checks a directory is usable
before anything is written into it. No module builds a path and writes
to it directly.

## Consequences

- One function to audit, and one place a new rule lands.
- `fuzz_path` can state the invariant for the whole crate: an accepted
  path never contains a `..` component, and sanitising an accepted path
  again returns it unchanged.
- Callers pay a check per write. That is not measurable against the I/O
  it precedes.

## Note (0.0.32)

Until 0.0.31, `sanitize_path` rewrote a path it could not honour
(dropping a `..` that climbed above the root, stripping disallowed
characters, removing a leading `/`) and returned the result as if it
were safe. From 0.0.32 it refuses such paths with an error, and its
containment rule is `euxis-commons`' `path::safe_join`. The "no module
builds a path and writes to it directly" part of the decision is not
yet implemented: no compiler module calls `sanitize_path`.
