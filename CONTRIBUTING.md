# Contributing

## Principles

- Keep crate reasons-to-change narrow.
- Put profile changes in `profiles/`, not semantic leaf crates.
- Prefer feature-backed changes over free-form edits.
- Keep receipts and schemas synchronized.
- Do not commit generated runtime output from `artifacts/`.

## License and CLA

- `xbrlkit` is licensed under `AGPL-3.0-or-later`.
- `AGPL-3.0-or-later` plus CLA is the intentional public contribution policy for this alpha series.
- All intentionally submitted contributions require the repository [CLA](./CLA.md).
- If the CLA Assistant comments on your pull request, reply with `I have read the CLA Document and I hereby sign the CLA`.
- If you contribute on behalf of an employer or other entity, make sure you have authority to agree to the CLA.

## Commands

### Maintainer Shortcuts

For quick iteration, use the Makefile wrappers:

```bash
make quick    # Run quality gates (fmt, clippy, test) - fast feedback
make full     # Run full alpha gate validation
```

See [docs/how-to/maintainer-commands.md](./docs/how-to/maintainer-commands.md) for comprehensive documentation on all maintainer commands, including:
- When to use `make quick` vs `make full`
- All `cargo xtask` commands with examples
- Common workflows and troubleshooting

### Direct Commands

```bash
cargo xtask doctor
cargo xtask feature-grid
cargo xtask schema-check
cargo xtask bdd --tags @alpha-active
cargo xtask test-ac AC-XK-SEC-INLINE-001
cargo xtask test-ac AC-XK-TAXONOMY-001
cargo xtask test-ac AC-XK-TAXONOMY-002
cargo xtask test-ac AC-XK-DUPLICATES-001
cargo xtask test-ac AC-XK-IXDS-001
cargo xtask test-ac AC-XK-IXDS-002
cargo xtask alpha-check
cargo test --workspace
```

For the current alpha surface, `cargo xtask test-ac` and `cargo xtask bdd --tags @alpha-active` both execute the same active slices through the shared scenario runner.
