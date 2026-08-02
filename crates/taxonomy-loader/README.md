# taxonomy-loader

XBRL taxonomy loading from XSD and linkbase files.

## Usage

```toml
[dependencies]
taxonomy-loader = "0.1.0-alpha.1"
```

HTTP loading is opt-in because it uses a blocking HTTP client. Enable the
`http` feature when loading taxonomies from `http://` or `https://` URLs:

```toml
[dependencies]
taxonomy-loader = { version = "0.1.0-alpha.1", features = ["http"] }
```

Without `http`, local filesystem loading remains available and URL inputs fail
closed as unsupported without constructing a network client.

See [xbrlkit](https://github.com/EffortlessMetrics/xbrlkit) for more.
