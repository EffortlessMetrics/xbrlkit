//! Shared scenario execution for repo-local developer flows.

use anyhow::{Context, anyhow};
use receipt_types::{Receipt, RunResult};
use scenario_contract::ScenarioRecord;
use sec_profile_types::{ProfilePack, load_profile_from_workspace};
use serde::Deserialize;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;
use validation_run::{
    TaxonomyResolutionRun, ValidationRun, resolve_taxonomy_entry_points, validate_duplicate_report,
    validate_html_members, validate_taxonomy_entry_points,
};
use xbrl_report_types::{CanonicalReport, Fact};

#[derive(Debug, Clone, Default)]
pub struct ScenarioExecution {
    pub validation_run: Option<ValidationRun>,
    pub taxonomy_resolution: Option<TaxonomyResolutionRun>,
    pub ixds_receipt: Option<Receipt>,
    pub export_receipt: Option<Receipt>,
}

#[derive(Debug, Deserialize)]
struct EntryPointsFixture {
    #[serde(default)]
    entry_points: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct ReportFixture {
    #[serde(default)]
    facts: Vec<Fact>,
}

const FIXTURE_CACHE_CAPACITY: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileFingerprint {
    modified: SystemTime,
    length: u64,
}

#[derive(Debug, Default)]
struct FixtureCache {
    entries: HashMap<PathBuf, (FileFingerprint, String)>,
    lru: VecDeque<PathBuf>,
}

impl FixtureCache {
    fn get(&mut self, path: &Path, fingerprint: FileFingerprint) -> Option<String> {
        let cached = self
            .entries
            .get(path)
            .and_then(|(cached_fingerprint, content)| {
                (cached_fingerprint == &fingerprint).then(|| content.clone())
            });
        if cached.is_some() {
            self.touch(path);
        } else if self.entries.contains_key(path) {
            self.remove(path);
        }
        cached
    }

    fn insert(&mut self, path: PathBuf, fingerprint: FileFingerprint, content: String) {
        self.remove(&path);
        self.entries.insert(path.clone(), (fingerprint, content));
        self.lru.push_back(path);
        while self.entries.len() > FIXTURE_CACHE_CAPACITY {
            let Some(evicted) = self.lru.pop_front() else {
                break;
            };
            self.entries.remove(&evicted);
        }
    }

    fn touch(&mut self, path: &Path) {
        self.lru.retain(|candidate| candidate != path);
        self.lru.push_back(path.to_path_buf());
    }

    fn remove(&mut self, path: &Path) {
        self.entries.remove(path);
        self.lru.retain(|candidate| candidate != path);
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.lru.clear();
    }
}

static FIXTURE_CACHE: OnceLock<Mutex<FixtureCache>> = OnceLock::new();

#[cfg(test)]
static FIXTURE_FILE_READ_COUNTS: OnceLock<Mutex<HashMap<PathBuf, usize>>> = OnceLock::new();

fn fixture_cache() -> &'static Mutex<FixtureCache> {
    FIXTURE_CACHE.get_or_init(|| Mutex::new(FixtureCache::default()))
}

/// Invalidate cached fixture content after an in-process fixture write.
///
/// The cache uses metadata to avoid repeated content reads. Callers that
/// mutate a fixture and then load it again in the same process must call this
/// function before the next load, including for same-size rewrites that may
/// preserve the metadata fingerprint.
///
/// # Errors
///
/// Returns an error if the process-local cache mutex is poisoned.
pub fn invalidate_fixture_cache() -> anyhow::Result<()> {
    fixture_cache()
        .lock()
        .map_err(|_| anyhow!("fixture cache mutex poisoned"))?
        .clear();
    Ok(())
}

fn read_fixture_file(path: &Path) -> anyhow::Result<String> {
    let fingerprint = file_fingerprint(path)?;
    if let Some(content) = fixture_cache()
        .lock()
        .map_err(|_| anyhow!("fixture cache mutex poisoned"))?
        .get(path, fingerprint)
    {
        return Ok(content);
    }

    #[cfg(test)]
    record_fixture_file_read(path)?;
    let content =
        fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    fixture_cache()
        .lock()
        .map_err(|_| anyhow!("fixture cache mutex poisoned"))?
        .insert(path.to_path_buf(), fingerprint, content.clone());
    Ok(content)
}

#[cfg(test)]
fn record_fixture_file_read(path: &Path) -> anyhow::Result<()> {
    let mut counts = FIXTURE_FILE_READ_COUNTS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|_| anyhow!("fixture read-count mutex poisoned"))?;
    *counts.entry(path.to_path_buf()).or_default() += 1;
    Ok(())
}

#[cfg(test)]
fn fixture_file_read_count(path: &Path) -> anyhow::Result<usize> {
    let counts = FIXTURE_FILE_READ_COUNTS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|_| anyhow!("fixture read-count mutex poisoned"))?;
    Ok(counts.get(path).copied().unwrap_or_default())
}

fn file_fingerprint(path: &Path) -> anyhow::Result<FileFingerprint> {
    let metadata = fs::metadata(path).with_context(|| format!("reading {}", path.display()))?;
    let modified = metadata
        .modified()
        .with_context(|| format!("reading metadata for {}", path.display()))?;
    Ok(FileFingerprint {
        modified,
        length: metadata.len(),
    })
}

pub fn execute_scenario(
    repo_root: &Path,
    scenario: &ScenarioRecord,
) -> anyhow::Result<ScenarioExecution> {
    let fixture_dirs = scenario
        .fixtures
        .iter()
        .map(|fixture| repo_root.join("fixtures").join(fixture))
        .collect::<Vec<_>>();
    if fixture_dirs.is_empty() {
        anyhow::bail!("scenario {} has no fixtures", scenario.scenario_id);
    }

    if fixture_dirs
        .iter()
        .all(|fixture_dir| fixture_dir.join("entrypoints.yaml").exists())
    {
        let profile = load_profile_for_scenario(repo_root, scenario)?;
        let entry_points = load_entry_points(&fixture_dirs)?;
        if scenario
            .receipts
            .iter()
            .any(|receipt| receipt == "taxonomy.resolve.v1")
        {
            return Ok(ScenarioExecution {
                taxonomy_resolution: Some(resolve_taxonomy_entry_points(&entry_points, &profile)),
                ..ScenarioExecution::default()
            });
        }
        return Ok(ScenarioExecution {
            validation_run: Some(validate_taxonomy_entry_points(&entry_points, &profile)),
            ..ScenarioExecution::default()
        });
    }

    if fixture_dirs
        .iter()
        .all(|fixture_dir| fixture_dir.join("report.yaml").exists())
    {
        let report = load_fixture_facts(&fixture_dirs)?;
        return Ok(ScenarioExecution {
            validation_run: Some(validate_duplicate_report(report)),
            ..ScenarioExecution::default()
        });
    }

    let profile = load_profile_for_scenario(repo_root, scenario)?;
    let owned_members = load_html_members(&fixture_dirs)?;
    let members = owned_members
        .iter()
        .map(|(member, html)| (member.as_str(), html.as_str()))
        .collect::<Vec<_>>();
    let validation_run = validate_html_members(&members, &profile);
    let ixds_receipt = scenario
        .receipts
        .iter()
        .any(|receipt| receipt == "ixds.assembly.v1")
        .then(|| ixds_assembly_receipt(&validation_run.report));
    let export_receipt = scenario
        .receipts
        .iter()
        .any(|receipt| receipt == "export.report.v1")
        .then(|| export_run::export_json(&validation_run.report).1);
    Ok(ScenarioExecution {
        validation_run: Some(validation_run),
        taxonomy_resolution: None,
        ixds_receipt,
        export_receipt,
    })
}

pub fn load_fixture_facts(fixture_dirs: &[PathBuf]) -> anyhow::Result<CanonicalReport> {
    let mut report = CanonicalReport::default();
    for fixture_dir in fixture_dirs {
        let path = fixture_dir.join("report.yaml");
        let content = read_fixture_file(&path)?;
        let fixture: ReportFixture = serde_yaml::from_str(&content)
            .with_context(|| format!("parsing {}", path.display()))?;
        report.members.push(fixture_dir.display().to_string());
        report.facts.extend(fixture.facts);
    }
    Ok(report)
}

pub fn load_html_members(fixture_dirs: &[PathBuf]) -> anyhow::Result<Vec<(String, String)>> {
    let mut members = Vec::new();
    for fixture_dir in fixture_dirs {
        let mut html_paths = fs::read_dir(fixture_dir)
            .with_context(|| format!("reading {}", fixture_dir.display()))?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "html")
            })
            .collect::<Vec<_>>();
        html_paths.sort();
        for path in html_paths {
            let html = read_fixture_file(&path)?;
            let member_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_string();
            members.push((member_name, html));
        }
    }
    if members.is_empty() {
        anyhow::bail!("no html members found in fixture directories");
    }
    Ok(members)
}

#[must_use]
pub fn ixds_assembly_receipt(report: &CanonicalReport) -> Receipt {
    let mut receipt = Receipt::new(
        "ixds.assembly",
        format!("{} members", report.members.len()),
        RunResult::Success,
    );
    receipt
        .notes
        .push(format!("assembled {} fact(s)", report.facts.len()));
    receipt
}

pub fn write_execution_receipts(
    repo_root: &Path,
    execution: &ScenarioExecution,
) -> anyhow::Result<()> {
    if let Some(validation_run) = &execution.validation_run {
        write_json(
            &repo_root.join("artifacts/validation/validation.report.v1.json"),
            &validation_run.receipt,
        )?;
    }
    if let Some(ixds_receipt) = &execution.ixds_receipt {
        write_json(
            &repo_root.join("artifacts/ixds/ixds.assembly.v1.json"),
            ixds_receipt,
        )?;
    }
    if let Some(taxonomy_resolution) = &execution.taxonomy_resolution {
        write_json(
            &repo_root.join("artifacts/taxonomy/taxonomy.resolve.v1.json"),
            &taxonomy_resolution.receipt,
        )?;
    }
    if let Some(export_receipt) = &execution.export_receipt {
        write_json(
            &repo_root.join("artifacts/export/export.report.v1.json"),
            export_receipt,
        )?;
    }
    Ok(())
}

pub fn assert_scenario_outcome(
    scenario: &ScenarioRecord,
    execution: &ScenarioExecution,
) -> anyhow::Result<()> {
    // Table-driven AC assertions — add new ACs here
    match scenario.ac_id.as_deref() {
        // SEC Inline Restrictions
        Some("AC-XK-SEC-INLINE-001") => {
            check_validation_rule(execution, "SEC.INLINE.NO_IX_FRACTION")
        }
        Some("AC-XK-SEC-INLINE-002") => check_validation_rule(execution, "SEC.INLINE.NO_IX_TUPLE"),

        // SEC Required Facts
        Some("AC-XK-SEC-REQUIRED-001") => {
            check_validation_rule(execution, "SEC.REQUIRED_FACT.DEI_ENTITYREGISTRANTNAME")
        }
        Some("AC-XK-SEC-REQUIRED-002") => ensure_report_has_no_error_findings(execution),

        // Taxonomy
        Some("AC-XK-TAXONOMY-001") => {
            ensure_taxonomy_resolution_succeeds(execution)?;
            ensure_taxonomy_resolution_resolves_at_least(execution, 1)
        }
        Some("AC-XK-TAXONOMY-002") => check_validation_rule(execution, "SEC.TAXONOMY.SAME_YEAR"),

        // Duplicates
        Some("AC-XK-DUPLICATES-001") => {
            check_validation_no_rule(execution, "XBRL.DUPLICATE_FACT.INCONSISTENT")
        }

        // IXDS Assembly
        Some("AC-XK-IXDS-001") => check_ixds_assembly(execution, 1),
        Some("AC-XK-IXDS-002") => check_ixds_assembly(execution, 2),

        // Export
        Some("AC-XK-EXPORT-001") => {
            if execution.export_receipt.is_none() {
                anyhow::bail!("export receipt was not emitted");
            }
            Ok(())
        }

        // Streaming Parser
        Some("AC-XK-STREAM-001") => {
            // Streaming parser memory validation - BDD steps handle assertions
            Ok(())
        }
        Some("AC-XK-STREAM-002") => {
            // Parser fallback logic - BDD steps handle assertions
            Ok(())
        }
        Some("AC-XK-STREAM-003") => {
            // Context validation during streaming - BDD steps handle assertions
            Ok(())
        }
        Some("AC-XK-STREAM-004") => {
            // Custom handler support - BDD steps handle assertions
            Ok(())
        }

        // Filing Manifest
        Some("AC-XK-MANIFEST-001") => {
            // BDD steps handle the assertions
            Ok(())
        }

        // Scenarios without AC ID use BDD step definitions
        None => Ok(()),

        _ => anyhow::bail!(
            "no scenario assertions implemented for {}",
            scenario.scenario_id
        ),
    }
}

/// Helper: Check validation report contains expected rule
fn check_validation_rule(execution: &ScenarioExecution, rule_id: &str) -> anyhow::Result<()> {
    let validation_run = execution
        .validation_run
        .as_ref()
        .context("missing validation run")?;
    ensure_report_contains_rule(validation_run, rule_id)
}

/// Helper: Check validation report does NOT contain rule
fn check_validation_no_rule(execution: &ScenarioExecution, rule_id: &str) -> anyhow::Result<()> {
    let validation_run = execution
        .validation_run
        .as_ref()
        .context("missing validation run")?;
    ensure_report_does_not_contain_rule(validation_run, rule_id)
}

/// Helper: Check IXDS assembly with expected member count and standard DEI concepts
fn check_ixds_assembly(
    execution: &ScenarioExecution,
    expected_members: usize,
) -> anyhow::Result<()> {
    ensure_ixds_member_count(execution, expected_members)?;
    ensure_report_fact_count(execution, 14)?;
    ensure_report_concept_set(
        execution,
        &[
            "dei:EntityRegistrantName",
            "dei:DocumentType",
            "dei:DocumentPeriodEndDate",
            "dei:AmendmentFlag",
            "dei:EntityCentralIndexKey",
            "dei:CurrentFiscalYearEndDate",
            "dei:DocumentAnnualReport",
            "dei:EntityAddressAddressLine1",
            "dei:EntityAddressCityOrTown",
            "dei:EntityAddressStateOrProvince",
            "dei:EntityAddressPostalZipCode",
            "dei:AuditorName",
            "dei:AuditorFirmId",
            "dei:AuditorLocation",
        ],
    )
}

pub fn ensure_report_contains_rule(
    validation_run: &ValidationRun,
    rule_id: &str,
) -> anyhow::Result<()> {
    if validation_run
        .report
        .findings
        .iter()
        .any(|finding| finding.rule_id == rule_id)
    {
        Ok(())
    } else {
        anyhow::bail!("validation report is missing expected rule {rule_id}")
    }
}

pub fn ensure_report_does_not_contain_rule(
    validation_run: &ValidationRun,
    rule_id: &str,
) -> anyhow::Result<()> {
    if validation_run
        .report
        .findings
        .iter()
        .any(|finding| finding.rule_id == rule_id)
    {
        anyhow::bail!("validation report unexpectedly contains rule {rule_id}")
    }
    Ok(())
}

pub fn ensure_report_has_no_error_findings(execution: &ScenarioExecution) -> anyhow::Result<()> {
    let validation_run = execution
        .validation_run
        .as_ref()
        .context("missing validation run")?;
    if validation_run
        .report
        .findings
        .iter()
        .any(|finding| finding.severity == "error")
    {
        anyhow::bail!("validation report contains unexpected error findings")
    }
    Ok(())
}

pub fn ensure_ixds_member_count(
    execution: &ScenarioExecution,
    expected: usize,
) -> anyhow::Result<()> {
    let validation_run = execution
        .validation_run
        .as_ref()
        .context("missing validation run for IXDS assertion")?;
    if validation_run.report.members.len() == expected {
        Ok(())
    } else {
        anyhow::bail!(
            "expected {} IXDS member(s), found {}",
            expected,
            validation_run.report.members.len()
        )
    }
}

pub fn ensure_taxonomy_resolution_succeeds(execution: &ScenarioExecution) -> anyhow::Result<()> {
    let taxonomy_resolution = execution
        .taxonomy_resolution
        .as_ref()
        .context("missing taxonomy resolution run")?;
    if taxonomy_resolution.receipt.result == RunResult::Success {
        Ok(())
    } else {
        anyhow::bail!("taxonomy resolution did not succeed")
    }
}

pub fn ensure_taxonomy_resolution_resolves_at_least(
    execution: &ScenarioExecution,
    expected: usize,
) -> anyhow::Result<()> {
    let taxonomy_resolution = execution
        .taxonomy_resolution
        .as_ref()
        .context("missing taxonomy resolution run")?;
    if taxonomy_resolution.dts.namespaces.len() >= expected {
        Ok(())
    } else {
        anyhow::bail!(
            "expected at least {} namespaces, found {}",
            expected,
            taxonomy_resolution.dts.namespaces.len()
        )
    }
}

pub fn ensure_report_fact_count(
    execution: &ScenarioExecution,
    expected: usize,
) -> anyhow::Result<()> {
    let validation_run = execution
        .validation_run
        .as_ref()
        .context("missing validation run")?;
    if validation_run.report.facts.len() == expected {
        Ok(())
    } else {
        anyhow::bail!(
            "expected {} facts, found {}",
            expected,
            validation_run.report.facts.len()
        )
    }
}

pub fn ensure_report_concept_set(
    execution: &ScenarioExecution,
    expected: &[&str],
) -> anyhow::Result<()> {
    let validation_run = execution
        .validation_run
        .as_ref()
        .context("missing validation run")?;
    let actual = validation_run
        .report
        .facts
        .iter()
        .map(|fact| fact.concept.clone())
        .collect::<BTreeSet<_>>();
    let expected = expected
        .iter()
        .map(|concept| (*concept).to_string())
        .collect::<BTreeSet<_>>();
    if actual == expected {
        Ok(())
    } else {
        anyhow::bail!("unexpected concept set: expected {expected:?}, found {actual:?}")
    }
}

fn load_profile_for_scenario(
    repo_root: &Path,
    scenario: &ScenarioRecord,
) -> anyhow::Result<ProfilePack> {
    let profile_pack = scenario
        .profile_pack
        .as_deref()
        .with_context(|| format!("scenario {} is missing profile_pack", scenario.scenario_id))?;
    load_profile_from_workspace(repo_root, profile_pack)
}

fn load_entry_points(fixture_dirs: &[PathBuf]) -> anyhow::Result<Vec<String>> {
    let mut entry_points = Vec::new();
    for fixture_dir in fixture_dirs {
        let path = fixture_dir.join("entrypoints.yaml");
        let content = read_fixture_file(&path)?;
        let fixture: EntryPointsFixture = serde_yaml::from_str(&content)
            .with_context(|| format!("parsing {}", path.display()))?;
        entry_points.extend(fixture.entry_points);
    }
    Ok(entry_points)
}

fn write_json(path: &Path, value: &impl serde::Serialize) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(value).context("serializing json")?;
    fs::write(path, bytes).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        FIXTURE_CACHE_CAPACITY, FileFingerprint, FixtureCache, fixture_cache,
        fixture_file_read_count, invalidate_fixture_cache, load_fixture_facts,
    };
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{Duration, SystemTime};

    static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

    struct TempFixtureDir(PathBuf);

    impl Drop for TempFixtureDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn temp_fixture_dir() -> Result<TempFixtureDir, String> {
        let id = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "xbrlkit-scenario-runner-{}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(&path).map_err(|error| error.to_string())?;
        Ok(TempFixtureDir(path))
    }

    fn fingerprint(seed: u64) -> FileFingerprint {
        FileFingerprint {
            modified: SystemTime::UNIX_EPOCH + Duration::from_secs(seed),
            length: seed,
        }
    }

    #[test]
    fn cache_returns_content_for_matching_fingerprint() -> Result<(), String> {
        let mut cache = FixtureCache::default();
        let path = PathBuf::from("fixture.yaml");
        let file_fingerprint = fingerprint(1);
        cache.insert(path.clone(), file_fingerprint, "cached".to_string());

        if cache.get(&path, file_fingerprint).as_deref() != Some("cached") {
            return Err("matching fixture should be served from cache".to_string());
        }
        Ok(())
    }

    #[test]
    fn cache_invalidates_changed_fingerprint() -> Result<(), String> {
        let mut cache = FixtureCache::default();
        let path = PathBuf::from("fixture.yaml");
        cache.insert(path.clone(), fingerprint(1), "stale".to_string());

        if cache.get(&path, fingerprint(2)).is_some() {
            return Err("changed fixture metadata must invalidate the cache".to_string());
        }
        if cache.entries.contains_key(&path) {
            return Err("invalidated fixture must be removed from the cache".to_string());
        }
        Ok(())
    }

    #[test]
    fn cache_evicts_least_recently_used_entry() -> Result<(), String> {
        let mut cache = FixtureCache::default();
        for index in 0..FIXTURE_CACHE_CAPACITY {
            let path = PathBuf::from(format!("fixture-{index}.yaml"));
            cache.insert(path, fingerprint(index as u64), index.to_string());
        }
        let oldest_path = Path::new("fixture-0.yaml");
        if cache.get(oldest_path, fingerprint(0)).is_none() {
            return Err("fixture 0 should be present before eviction".to_string());
        }
        cache.insert(
            PathBuf::from("fixture-new.yaml"),
            fingerprint(100),
            "new".to_string(),
        );

        if cache.get(oldest_path, fingerprint(0)).is_none() {
            return Err("recently used fixture should not be evicted".to_string());
        }
        if cache
            .get(Path::new("fixture-1.yaml"), fingerprint(1))
            .is_some()
        {
            return Err("least-recently-used fixture should be evicted".to_string());
        }
        Ok(())
    }

    #[test]
    fn loader_rereads_fixture_when_file_length_changes() -> Result<(), String> {
        let fixture_dir = temp_fixture_dir()?;
        let report_path = fixture_dir.0.join("report.yaml");
        fs::write(&report_path, "facts: []\n").map_err(|error| error.to_string())?;
        let first = load_fixture_facts(std::slice::from_ref(&fixture_dir.0))
            .map_err(|error| error.to_string())?;
        if !first.facts.is_empty() {
            return Err("initial fixture should contain no facts".to_string());
        }

        fs::write(
            &report_path,
            "facts:\n  - concept: test:Fact\n    context: c1\n    value: \"1\"\n",
        )
        .map_err(|error| error.to_string())?;
        let second = load_fixture_facts(std::slice::from_ref(&fixture_dir.0))
            .map_err(|error| error.to_string())?;
        if second.facts.len() != 1 {
            return Err("changed fixture should be reread instead of cached".to_string());
        }
        Ok(())
    }

    #[test]
    fn loader_reads_unchanged_fixture_once() -> Result<(), String> {
        let fixture_dir = temp_fixture_dir()?;
        let report_path = fixture_dir.0.join("report.yaml");
        fs::write(&report_path, "facts: []\n").map_err(|error| error.to_string())?;
        let before = fixture_file_read_count(&report_path).map_err(|error| error.to_string())?;

        load_fixture_facts(std::slice::from_ref(&fixture_dir.0))
            .map_err(|error| error.to_string())?;
        load_fixture_facts(std::slice::from_ref(&fixture_dir.0))
            .map_err(|error| error.to_string())?;

        let after = fixture_file_read_count(&report_path).map_err(|error| error.to_string())?;
        if after - before != 1 {
            return Err(format!(
                "unchanged fixture should be read once, observed {} reads",
                after - before
            ));
        }
        Ok(())
    }

    #[test]
    fn loader_rereads_fixture_after_explicit_invalidation() -> Result<(), String> {
        let fixture_dir = temp_fixture_dir()?;
        let report_path = fixture_dir.0.join("report.yaml");
        fs::write(
            &report_path,
            "facts:\n  - concept: test:Fact\n    context: c1\n    value: \"1\"\n",
        )
        .map_err(|error| error.to_string())?;

        let first = load_fixture_facts(std::slice::from_ref(&fixture_dir.0))
            .map_err(|error| error.to_string())?;
        if first.facts.first().map(|fact| fact.value.as_str()) != Some("1") {
            return Err("initial fixture should contain value 1".to_string());
        }

        fs::write(
            &report_path,
            "facts:\n  - concept: test:Fact\n    context: c1\n    value: \"2\"\n",
        )
        .map_err(|error| error.to_string())?;
        invalidate_fixture_cache().map_err(|error| error.to_string())?;

        let second = load_fixture_facts(std::slice::from_ref(&fixture_dir.0))
            .map_err(|error| error.to_string())?;
        if second.facts.first().map(|fact| fact.value.as_str()) != Some("2") {
            return Err("explicit invalidation should expose rewritten fixture".to_string());
        }
        Ok(())
    }

    #[test]
    fn explicit_invalidation_discards_matching_metadata_entry() -> Result<(), String> {
        let path = PathBuf::from(format!(
            "same-fingerprint-{}-{}.yaml",
            std::process::id(),
            NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        let file_fingerprint = fingerprint(17);
        fixture_cache()
            .lock()
            .map_err(|_| "fixture cache mutex poisoned".to_string())?
            .insert(path.clone(), file_fingerprint, "stale".to_string());

        invalidate_fixture_cache().map_err(|error| error.to_string())?;

        let cached = fixture_cache()
            .lock()
            .map_err(|_| "fixture cache mutex poisoned".to_string())?
            .get(&path, file_fingerprint);
        if cached.is_some() {
            return Err("explicit invalidation must discard matching metadata entries".to_string());
        }
        Ok(())
    }

    #[test]
    fn missing_fixture_keeps_read_error_context() -> Result<(), String> {
        let fixture_dir = temp_fixture_dir()?;
        let error = match load_fixture_facts(std::slice::from_ref(&fixture_dir.0)) {
            Ok(_) => return Err("missing fixture should fail".to_string()),
            Err(error) => error.to_string(),
        };
        if !error.contains("report.yaml") || !error.contains("reading") {
            return Err(format!("missing-fixture context was lost: {error}"));
        }
        Ok(())
    }

    #[test]
    fn malformed_fixture_keeps_parse_error_context() -> Result<(), String> {
        let fixture_dir = temp_fixture_dir()?;
        let report_path = fixture_dir.0.join("report.yaml");
        fs::write(&report_path, "facts: [").map_err(|error| error.to_string())?;
        let error = match load_fixture_facts(std::slice::from_ref(&fixture_dir.0)) {
            Ok(_) => return Err("malformed fixture should fail".to_string()),
            Err(error) => error.to_string(),
        };
        if !error.contains("report.yaml") || !error.contains("parsing") {
            return Err(format!("malformed-fixture context was lost: {error}"));
        }
        Ok(())
    }
}
