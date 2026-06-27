# automake-rs build-atlas

A **build-profiler atlas**: every point in the build corpus is turned into a reproducible, versioned
recipe (`recipes/<owner>__<name>.json`). Each recipe pins the source (git SHA), the GNU-free
toolchain, the **probe results** (config.h `HAVE_*`), **feature flags**, the **dependency-graph
snapshot** (system libs / pkg-config / headers needed / missing), the **optimal pass pipeline** (the
steps that actually work), **target settings**, **known quirks** (libtool / pkg-config / gettext /
autoconf-archive / subdirs / required `LIBS`), and the **verified outputs** (sha256).

The point: a future build *reads the recipe* — installs the recorded deps, applies the known
pipeline and quirks, and reproduces the verified output — instead of rediscovering everything.
This is the cache that makes build success compound.

- **GNU-free**: only `autoreconf-rs` / `acrs-*` are invoked; `toolchain.gnu_free: true` asserts no
  GNU autotools binary ran.
- **Schema**: see [SCHEMA.md](SCHEMA.md) (`automake-rs.build-atlas/v1`).
- **Index**: `recipes/INDEX.json` aggregates status counts + per-repo verdicts.
- **Regenerate**: `cargo xtask atlas <corpus-list> [out-dir]` (the generator is `xtask/src/atlas.rs`
  — Rust only, no scripts).
