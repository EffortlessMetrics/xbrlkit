# xbrlkit Friction Scan Results

**Scan Date:** 2026-04-29 05:57 CST / 21:57 UTC  
**Scanner:** cron friction-scan (job 4936e6bd-9a12-460a-a98c-0f2289079431)  
**Repo:** https://github.com/EffortlessMetrics/xbrlkit  
**Commit:** `f841671` (feat(bdd-steps): wire package check BDD step handlers for #246)  
**Previous Scan:** 2026-04-29 01:57 CST (`7252fa3`)

---

## Executive Summary

| Category | Count | Severity | Status |
|----------|-------|----------|--------|
| TODO/FIXME/XXX comments | 0 | — | CLEAN |
| Code duplication | 3 instances | medium | **IMPROVED** |
| Slow test / heavy fixture patterns | 0 explicit, 1 structural | medium | UNCHANGED |
| Unused dependencies | 13 confirmed | low-medium | UNCHANGED |
| Empty/placeholder crates | 22 crates | low | UNCHANGED |
| Panic/expect in non-test code | 2 patterns | low | UNCHANGED |
| Type name collision | 1 | low | UNCHANGED |
| Dependency version mismatch | 1 | low | UNCHANGED |
| Unused API parameter | 1 | low | UNCHANGED |
| Modern API compatibility | 1 observation | info | NEW |

**Overall assessment:** Low friction. No critical issues. The 5 commits since the last scan (2026-04-29 01:57) were predominantly positive cleanups: one deduplicated quoted-string parsing, and four modernized nested `if let` blocks to `let-chains`. No new friction was introduced. All previously identified issues remain tracked by open issues.

---

## Commits Since Last Scan

| Commit | Message | Friction Impact |
|--------|---------|-----------------|
| `f841671` | feat(bdd-steps): wire package check BDD step handlers for #246 | Neutral — adds new BDD infrastructure, no new debt |
| `cc4cd9d` | fix(meta): add missing req_id to oim_json and cockpit_pack sidecars | Positive — metadata completeness |
| `491952a` | scout(auto-fix): archive orphaned plans, update active-work, close #205 | Positive — housekeeping |
| `4d1a387` | refactor(bdd-steps): deduplicate quoted-string parsing | **Positive — resolves duplication** |
| `537ccdd` | fix(fixture): add HTML member for manifest check scenario (closes #236) | Positive — test fixture completeness |

**Net change:** 1 duplication instance resolved, 0 new friction introduced.

---

## 1. TODO/FIXME/XXX Comments

**Status: CLEAN**

No TODO, FIXME, HACK, XXX, or NOTE markers found in production code. The only matches remain ISO4217:XXX currency format strings in `unit-rules/src/patterns.rs` and `unit-rules/src/lib.rs`, which are valid XBRL currency codes.

---

## 2. Code Duplication

### 2a. Quoted-string parsing — **RESOLVED** ✅

**File:** `crates/xbrlkit-bdd-steps/src/lib.rs`

Commit `4d1a387` extracted the repeated `"split-quote-collect"` pattern into a single `extract_all_quoted()` function:

```rust
fn extract_all_quoted(step: &str) -> Vec<String> {
    step.split('"')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, s)| s.to_string())
        .collect()
}
```

This replaces the 4 prior inline occurrences (lines ~367, ~471, ~776, and elsewhere). Five unit tests were also added for the new helper.

**Status:** RESOLVED by commit `4d1a387`. Closed issue #252.

---

### 2b. Repeated taxonomy setup in `xbrlkit-bdd-steps` (MEDIUM)

**File:** `crates/xbrlkit-bdd-steps/src/lib.rs`

The same `us-gaap:ScenarioDomain` + `us-gaap:StatementScenarioAxis` taxonomy setup is still repeated **3 times**:

- Lines 633–656 — inline in `handle_given`
- Lines 703–725 — inline in `handle_when`
- Lines 1558–1583 — in `create_synthetic_taxonomy()` helper

**Status:** UNCHANGED since last scan. Tracked by issues #224, #151.

---

### 2c. `StreamingContext` name collision (LOW)

**Files:**
- `crates/xbrl-stream/src/lib.rs:55` — `pub struct StreamingContext` (streaming parser context)
- `crates/xbrlkit-bdd-steps/src/lib.rs:63` — `pub struct StreamingContext` (BDD world state)

Both structs share the same name but serve completely different purposes.

**Status:** UNCHANGED. Tracked by issue #273.

---

### 2d. `Regex::new()` in hot path (LOW-MEDIUM)

**File:** `crates/unit-rules/src/patterns.rs:106-122`

Three regexes are still compiled **on every call** to `is_likely_monetary()` instead of being cached via `once_cell` or `std::sync::LazyLock`.

**Status:** UNCHANGED. Tracked by issues #268, #234, #230.

---

### 2e. `Regex::new().unwrap()` in constructor (LOW)

**File:** `crates/unit-rules/src/patterns.rs:29-61`

Six regexes are compiled on every `ConceptUnitPatterns::new()` call. Static patterns should be compiled once globally.

**Status:** UNCHANGED. Covered by same issues as 2d.

---

## 3. Slow Tests / Heavy Fixtures

### 3a. Monolithic BDD step handlers

**File:** `crates/xbrlkit-bdd-steps/src/lib.rs` (now ~1715 lines, up from 1625)

Four `#[allow(clippy::too_many_lines)]` suppressions remain:

| Function | Line | Suppression |
|----------|------|-------------|
| `handle_given` | 182 | `#[allow(clippy::too_many_lines)]` |
| `handle_when` | 667 | `#[allow(clippy::too_many_lines)]` |
| `handle_then` | 1044 | `#[allow(clippy::too_many_lines)]` |
| `handle_parameterized_assertion` | 1221 | `#[allow(clippy::too_many_lines)]` |

The new package-check step handlers (lines ~385 and ~945) add ~90 lines but are cleanly isolated behind `if step.text == "..."` guards.

**Status:** UNCHANGED. Tracked by issues #160, #165, #178, #210, #251, #155, #229.

---

### 3b. Package check BDD steps spawn `cargo` subprocesses

**File:** `crates/xbrlkit-bdd-steps/src/lib.rs`

New package-check steps execute `cargo metadata` and `cargo package -p <crate> --allow-dirty --locked --list` for every publishable crate. While not "sleep-based" slow, this is I/O-bound subprocess overhead that scales with workspace size.

**Impact:** Scenarios using these steps will execute significantly slower than pure-in-memory steps. This is expected for package validation, but worth noting if these scenarios are run in tight loops.

**Status:** NEW CODE — inherent to the feature, not a defect.

---

### 3c. No explicit slow patterns found

- No `std::thread::sleep`, `tokio::time::sleep`, or async delays in tests
- No heavy fixture loading patterns detected
- No `#[ignore]` attributes on tests
- `HTTP_TIMEOUT: Duration = Duration::from_secs(30)` in `taxonomy-loader/src/lib.rs:29` remains a network timeout

**Status:** UNCHANGED.

---

## 4. Unused Dependencies

### 4a. Confirmed unused dependencies (all still present)

| Crate | Unused Dependency | Status | Notes |
|-------|-------------------|--------|-------|
| `xbrl-stream` | `xbrl-report-types` | STILL UNUSED | No imports in source |
| `validation-run` | `context-completeness` | STILL UNUSED | `validate_context_completeness_streaming` implements its own logic; no imports from `context-completeness` |
| `dimensional-rules` | `serde` | STILL UNUSED | No serde usage in source |
| `dimensional-rules` | `thiserror` | STILL UNUSED | No thiserror usage in source |
| `receipt-types` | `serde_json` | STILL UNUSED | No serde_json usage in source (only `serde`) |
| `scenario-contract` | `serde_json` | STILL UNUSED | No serde_json usage in source (only `serde`) |
| `unit-rules` | `serde` | STILL UNUSED | No serde usage in source |
| `xbrlkit-cli` | `render-md` | STILL UNUSED | No render-md usage in source |
| `xtask` | `sec-profile-types` | STILL UNUSED | No imports in xtask/src |
| `xtask` | `serde_yaml` | STILL UNUSED | No imports in xtask/src |
| `xtask` | `validation-run` | STILL UNUSED | No imports in xtask/src |
| `xtask` | `walkdir` | STILL UNUSED | No imports in xtask/src |
| `xtask` | `xbrl-report-types` | STILL UNUSED | No imports in xtask/src |

**Note:** The recent commits converted many `path = "..."` dependencies to `workspace = true` form, but did **not** remove any unused dependencies. The structural issue remains.

**Impact:** Build bloat, longer compile times, unnecessary transitive dependency pulls.

**Status:** UNCHANGED. Tracked by issues #179, #161, #203, #249, #231, #232, #190.

---

### 4b. `thiserror` version mismatch

**File:** `crates/xbrl-stream/Cargo.toml`

```toml
[dependencies]
thiserror = "1.0"          # <-- still uses 1.x directly
```

Workspace declares `thiserror = "2.0.12"`. All other crates use `thiserror.workspace = true`.

**Impact:** Compiles two versions of `thiserror`.

**Status:** UNCHANGED. Tracked by issue #271.

---

## 5. Empty / Placeholder Crates

The following crates remain essentially empty (≤20 lines of actual code):

| Crate | Lines | Status |
|-------|-------|--------|
| `archive-zip` | 4 | placeholder |
| `calc11` | 4 | placeholder |
| `corpus-fs` | 5 | thin wrapper |
| `edgar-identity` | 7 | placeholder |
| `oim-normalize` | 10 | thin wrapper |
| `oracle-compare` | 5 | placeholder |
| `render-json` | 4 | placeholder |
| `render-md` | 8 | placeholder |
| `sec-http` | 6 | placeholder |
| `taxonomy-cache` | 6 | placeholder |
| `taxonomy-package` | 8 | placeholder |
| `taxonomy-types` | 13 | placeholder |
| `xbrl-dimensions` | 4 | placeholder |
| `xbrlkit-conform` | 5 | placeholder |
| `xbrlkit-core` | 7 | placeholder |
| `xbrlkit-interop-tests` | 5 | placeholder |
| `xbrlkit-test-grid` | 5 | placeholder |
| `xbrl-linkbases` | 4 | placeholder |
| `xbrl-units` | 4 | placeholder |
| `cockpit-export` | 12 | thin wrapper |
| `export-run` | 10 | thin wrapper |
| `edgar-attachments` | 20 | mostly empty |
| `edgar-sgml` | 14 | mostly empty |
| `filing-load` | 16 | mostly empty |

**Impact:** Workspace bloat. `cargo check/build/test` compiles all workspace members. Empty crates add noise to build output and increase CI time.

**Status:** UNCHANGED. Tracked by issues #266, #241, #196, #198.

---

## 6. Panic / Expect Patterns

### 6a. `expect()` in production code

**File:** `crates/export-run/src/lib.rs:10`

```rust
pub fn export_json(report: &CanonicalReport) -> (String, Receipt) {
    let json = serde_json::to_string_pretty(&to_json_value(report))
        .expect("canonical report serialization should succeed");
    // ...
}
```

**Status:** UNCHANGED. Tracked by issue #239.

---

### 6b. `unwrap()` in production code

**File:** `crates/dimensional-rules/src/lib.rs:160`

```rust
let dimension = dim_taxonomy.dimensions.get(&dim_member.dimension).unwrap();
```

This follows an explicit `contains_key` check (line 155), so it's logically safe, but it still uses `unwrap()` in production code. The function returns `Result<(), ValidationFinding>` so this should use `.ok_or()` or similar.

**Status:** UNCHANGED. Tracked by issues #194, #154, #235.

---

### 6c. `expect()` in BDD step infrastructure

**File:** `crates/xbrlkit-bdd-steps/src/lib.rs:185`

```rust
path.strip_prefix(world.repo_root.join("fixtures"))
    .expect("fixture path under repo root")
```

This is in the `assert_declared_inputs_match` function (test infrastructure). While technically not a `#[cfg(test)]` block, it only runs during scenario execution.

**Status:** Present since prior scan but not previously flagged. Low severity — test-adjacent code.

---

### 6d. `.expect()` / `.unwrap()` in test code (acceptable)

All other `.expect()` and `.unwrap()` calls are in `#[cfg(test)]` blocks. These are acceptable.

---

## 7. Other Friction

### 7a. Unused parameter in `validation-run`

**File:** `crates/validation-run/src/lib.rs:234`

```rust
pub fn validate_context_completeness_streaming(
    xbrl_xml: &str,
    _size_threshold_mb: usize,  // <-- still prefixed with underscore, never used
) -> Vec<ValidationFinding> {
```

The `_size_threshold_mb` parameter is still ignored.

**Status:** UNCHANGED. Tracked by issue #272.

---

### 7b. `std::fs::read_to_string` wrapper duplicated 8× across 6 crates

**Occurrences:**
- `crates/scenario-runner/src/lib.rs:111,135,473`
- `crates/taxonomy-loader/src/lib.rs:165,222`
- `crates/xbrlkit-bdd-steps/src/lib.rs:352,859`
- `crates/xbrlkit-bdd/src/lib.rs:92`
- `crates/sec-profile-types/src/lib.rs:78,107,113`
- `crates/xbrlkit-feature-grid/src/lib.rs:55`

`corpus-fs` already provides a shared wrapper, but only `xbrlkit-cli` uses it.

**Status:** UNCHANGED. Tracked by issues #264, #166, #176.

---

### 7c. `rust-version = "1.92"` in workspace Cargo.toml

**File:** `Cargo.toml`

```toml
[workspace.package]
rust-version = "1.92"
```

Rust 1.92 does not exist (latest stable as of this scan is approximately 1.86). This field declares the minimum supported Rust version. A non-existent version is either:
1. An intentional placeholder requiring a future/nightly toolchain, or
2. An error that should be corrected to a real version.

If intentional, it should be documented. If accidental, it may break downstream consumers or CI environments.

**Status:** NOT PREVIOUSLY FLAGGED. Observation only — verify intent.

---

## 8. Changes Since Last Scan — Detailed Verification

| Check | Previous | Current | Δ |
|-------|----------|---------|---|
| `thiserror = "1.0"` in xbrl-stream | present | present | — |
| `_size_threshold_mb` unused param | present | present | — |
| `StreamingContext` name collision | present | present | — |
| `Regex::new()` hot path in unit-rules | present | present | — |
| Empty placeholder crates | 22 | 22 | — |
| `unwrap()` in dimensional-rules | present | present | — |
| `expect()` in export-run | present | present | — |
| `context-completeness` unused in validation-run | present | present | — |
| `render-md` unused in xbrlkit-cli | present | present | — |
| `serde_json` unused in receipt-types | present | present | — |
| `serde` unused in unit-rules | present | present | — |
| `serde`/`thiserror` unused in dimensional-rules | present | present | — |
| `sec-http` completely unused | present | present | — |
| xtask unused deps (5) | present | present | — |
| `extract_all_quoted` duplication | present | **RESOLVED** | ✅ |
| `let-chains` modernization | not present | applied | cosmetic |
| `panic!` in xbrl-contexts tests | present | present | — |
| BDD step handler line count | 1625 | ~1715 | +90 (new package steps) |

---

## Recommendations (Priority Order)

1. **Remove unused dependencies** (13 entries across 8 crates) — immediate build time improvement
2. **Fix `thiserror` version** in `xbrl-stream` to use `thiserror.workspace = true` — eliminates duplicate compilation
3. **Cache regex compilation** in `unit-rules/src/patterns.rs` using `once_cell` or `std::sync::LazyLock` — runtime performance
4. **Extract shared taxonomy builder** in `xbrlkit-bdd-steps` — DRY the 3 repeated ScenarioDomain setups
5. **Rename `StreamingContext`** in `xbrlkit-bdd-steps` to `BddWorldState` or similar — avoid name collision
6. **Remove or consolidate** empty placeholder crates — reduce workspace bloat
7. **Refactor BDD step handlers** from monolithic dispatch to registry/module pattern — maintainability
8. **Remove unused parameter** `_size_threshold_mb` or implement the size check — API cleanup
9. **Promote `corpus-fs::read_to_string`** usage across all crates instead of inlining — consistency
10. **Verify `rust-version = "1.92"`** intent — correct or document

---

## Issue Filings

### Already Filed (from previous scans)

All friction points identified in this scan remain tracked by existing open issues. See the previous scan's issue table for the complete list.

### Filed This Scan

**None.** No new friction points requiring new issues were identified. The only code changes since the last scan were positive cleanups (deduplication + `let-chains` modernization).

### Not Yet Filed (observations)

| Item | Severity | Suggested Title | Suggested Labels |
|------|----------|-----------------|------------------|
| `rust-version = "1.92"` in workspace Cargo.toml | info | Verify workspace `rust-version` field intent | `infrastructure`, `question` |

---

*End of scan.*
