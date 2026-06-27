# automake-rs build-atlas — recipe schema v1

Every corpus point becomes a reproducible, versioned recipe. One JSON file per repo under
`recipes/<owner>__<name>.json`. The atlas turns hard-won build knowledge into a cache: a future
build reads the recipe, installs the recorded deps, applies the known pass pipeline and quirks, and
reproduces the verified output — instead of rediscovering everything.

```jsonc
{
  "schema": "automake-rs.build-atlas/v1",
  "repo": "owner/name",
  "source":   { "url", "git_sha", "snapshot_utc" },     // reproducible pin
  "toolchain":{ "autoconf_rs", "automake_rs", "m4_rs_core", "gnu_free": true },
  "target":   { "cc", "cflags", "host" },                // target settings
  "pass_pipeline": [ {"step","tool","status"} ... ],     // optimal pass pipeline (the steps that work)
  "probe_results": { "HAVE_*": 1, ... },                 // config.h feature probe outcomes
  "feature_flags": { "configure_args": [...] },          // feature flags
  "dependencies": { "pkg_config":[], "system_libs":[], "headers_needed":[], "missing":[] }, // dep graph snapshot
  "quirks": [ "human-readable known quirks / workarounds applied" ],
  "outputs": [ {"path","sha256","kind"} ... ],           // verified outputs
  "status": "FUNC_OK | CONFIGURE_FAIL | MAKE_FAIL | CLONE_FAIL | NO_AC",
  "verified": true|false
}
```

Recipes are GNU-free (autoreconf-rs / acrs-* only). `gnu_free:true` asserts no GNU autotools binary
was invoked. Regenerate with `atlas/atlas-builder.sh`.
