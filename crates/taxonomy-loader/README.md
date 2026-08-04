# taxonomy-loader

XBRL taxonomy loading from XSD and linkbase files.

## Usage

```toml
[dependencies]
taxonomy-loader = "0.1.0-alpha.1"
```

See [xbrlkit](https://github.com/EffortlessMetrics/xbrlkit) for more.

## Testing contract

Normal unit, focused acceptance, and BDD runs are local-only and must not
contact public taxonomy servers. Remote-loading tests use the loader's private
deterministic transport seam to exercise successful responses, HTTP status
failures, transport failures, and unsupported URL schemes without network
access.

The default production path continues to use the blocking `reqwest` client
when an HTTP or HTTPS taxonomy URL is loaded. The local-only test rule does not
change production HTTP behavior or the public loader constructors.

## Verification

From the repository root, run the focused unit and acceptance checks:

```text
cargo test -p taxonomy-loader --locked --offline
cargo xtask test-ac AC-XK-TAX-LOAD-005
cargo xtask test-ac AC-XK-TAX-LOAD-006
```

These commands exercise the deterministic transport, cache miss/write/hit, and
recursive schema-import paths without contacting public taxonomy servers.
