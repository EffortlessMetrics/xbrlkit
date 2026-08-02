# export-run

XBRL report export and serialization.

`export_json` returns `Result<(String, Receipt), ExportError>`. Callers must
handle serialization failures instead of assuming report export cannot fail.

## Usage

```toml
[dependencies]
export-run = "0.1.0-alpha.1"
```

See [xbrlkit](https://github.com/EffortlessMetrics/xbrlkit) for more.
