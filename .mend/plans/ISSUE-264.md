# Centralize corpus file reads — Issue #264

**Status:** Ready for review
**Issue:** #264
**Stream:** Refactor / maintenance

## Objective

Use the existing `corpus-fs::read_to_string` adapter for the remaining exact
copies of the repository's standard file-read error context. This keeps the
error wording and filesystem behavior in one implementation without changing
the public APIs or the profile-specific error messages.

## Current scope

Fresh inspection of `origin/main` found seven exact duplicate call sites in
four consuming crates:

- `sec-profile-types`: `read_yaml` and `read_standard_taxonomy_uris`;
- `xbrlkit-bdd`: `parse_feature_file`;
- `xbrlkit-feature-grid`: sidecar loading.
- `scenario-runner`: report fixtures, HTML members, and entry-point fixtures.

The profile loader's `reading profile pack {profile_id}` message and the
`xbrlkit-bdd-steps` submission-specific message are intentionally out of scope.
The issue's older seven-call-site count included stale or non-matching sites.

## Acceptance criteria

- [x] All seven exact duplicate reads use `corpus_fs::read_to_string`.
- [x] The consuming crates declare the existing `corpus-fs` workspace dependency.
- [x] Profile-specific and submission-specific error messages remain unchanged.
- [x] Focused package tests, formatting, Clippy, and workspace tests pass.
- [x] No runtime behavior or public API changes are introduced.

## Proof

```text
cargo test -p sec-profile-types -p xbrlkit-bdd -p xbrlkit-feature-grid --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
rg -n 'read_to_string\\([^;]*\\)\\.with_context' crates
git diff --check
```

The grep result may retain the canonical implementation in `corpus-fs` and
the intentionally excluded custom-context call sites; the changed consumers
are verified by direct source inspection and compilation.

## Non-goals

- Do not change `corpus-fs` error wording or filesystem behavior.
- Do not replace custom error contexts that carry domain-specific information.
- Do not remove `anyhow` from crates that still use `Context` elsewhere.
- Do not add drift-prevention tooling in this refactor PR.

## Rollback

Revert the dependency additions and the four mechanical source substitutions
across three source files;
no persisted data or external state is affected.
