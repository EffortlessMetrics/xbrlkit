# Issue #469: Preserve single-quoted iXBRL attributes

## Problem

`ixhtml-scan` only tracks double-quoted attributes while finding tag boundaries and parsing values. A `>` inside a single-quoted attribute therefore truncates the tag and corrupts the projected fragment.

## Target seam

- `crates/ixhtml-scan/src/lib.rs`
- Existing governed IXDS fixture `fixtures/synthetic/inline/ixds-single-file-01/member-a.html`

## Acceptance criteria

- Tag scanning treats both single and double quotes as balanced delimiters.
- Attribute parsing uses the matching quote character for quoted values.
- Existing double-quoted behavior and public types remain unchanged.
- A focused unit regression covers a single-quoted value containing `>`.
- The existing `SCN-XK-IXDS-001` BDD path passes with the fixture variation.

## Proof

```text
cargo test -p ixhtml-scan -p efm-rules -p ixds-assemble -p validation-run --locked --offline
cargo clippy -p ixhtml-scan -p efm-rules -p ixds-assemble -p validation-run --all-targets --locked --offline -- -D warnings
cargo xtask bdd --tags @SCN-XK-SEC-INLINE-001
cargo xtask bdd --tags @SCN-XK-IXDS-001
cargo xtask bdd --tags @SCN-XK-IXDS-002
cargo fmt --all -- --check
git diff --check
```

## Non-goals and rollback

This slice does not add an HTML parser dependency, change namespaces, or redesign iXBRL projection. Revert the focused source, fixture, and plan changes to roll back without migration.

## Claim boundary

The slice establishes balanced single/double quote handling for this scanner and its existing IXDS consumer path. It does not claim full HTML conformance or broader iXBRL parsing coverage.
