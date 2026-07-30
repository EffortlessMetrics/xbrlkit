# Coverage

Codecov coverage is Rust execution-surface evidence.

It answers:
> Did tests execute this Rust surface?

It does not answer:
- whether XBRL validation is correct,
- whether SEC EFM coverage is complete,
- whether EDGAR filing handling is complete,
- whether taxonomy loading is correct for broad real-world filings,
- whether inline XBRL document sets are complete,
- whether oracle comparison is complete,
- whether BDD coverage is complete,
- whether package readiness is proven,
- whether release readiness is proven.

Those are separate proof lanes.

## Workflow

The Coverage workflow runs on:
- push to `main`,
- `workflow_dispatch`,
- PRs labeled `coverage`, `full-ci`, or `ci:full`.

The initial Codecov flag is `rust-alpha`, scoped to Rust execution coverage for the alpha workspace.

## Receipts

Codecov comments are disabled. Durable receipts are:
- `coverage.json`,
- `coverage.txt`,
- `lcov.info`,
- the GitHub Actions coverage artifact,
- the Codecov dashboard.
