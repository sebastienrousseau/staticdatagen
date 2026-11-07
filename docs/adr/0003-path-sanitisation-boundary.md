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
were safe, and no compiler module called it: the compiler joined page
names under the build directory with a plain `Path::join`, so a
`FileData` named `../x.md` or `/abs/x.md` passed to
`write_files_to_build_directory` was written outside it.

From 0.0.32 the boundary is the containment rule rather than one
function: every path built from content, front matter or configuration
is joined under its trusted root with `euxis-commons`'
`path::safe_join`, and refused if it is absolute or climbs above the
root. The write path, `move_output_directory_to`, navigation and
`models::data::validation::join_under` all use it; `sanitize_path`
refuses instead of rewriting. `docs/ARCHITECTURE.md` lists each site.
