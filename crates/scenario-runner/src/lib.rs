//! Shared scenario execution for repo-local developer flows.

use anyhow::Context;
use receipt_types::{Receipt, RunResult};
use scenario_contract::{FeatureGrid, ScenarioRecord};
use sec_profile_types::{ProfilePack, load_profile_from_workspace};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
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
    pub filing_receipt: Option<Receipt>,
    pub feature_grid: Option<FeatureGrid>,
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

pub fn execute_scenario(
    repo_root: &Path,
    scenario: &ScenarioRecord,
) -> anyhow::Result<ScenarioExecution> {
    // Feature grid compilation (no fixtures needed)
    if scenario
        .receipts
        .iter()
        .any(|receipt| receipt == "feature.grid.v1")
    {
        let grid = xbrlkit_feature_grid::compile(repo_root)?;
        return Ok(ScenarioExecution {
            feature_grid: Some(grid),
            ..ScenarioExecution::default()
        });
    }

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

    if fixture_dirs
        .iter()
        .all(|fixture_dir| fixture_dir.join("submission.txt").exists())
    {
        let submission = load_submission(&fixture_dirs)?;
        let (_manifest, receipt) = filing_load::load_from_submission(&submission);
        return Ok(ScenarioExecution {
            filing_receipt: Some(receipt),
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
        filing_receipt: None,
        feature_grid: None,
    })
}

pub fn load_fixture_facts(fixture_dirs: &[PathBuf]) -> anyhow::Result<CanonicalReport> {
    let mut report = CanonicalReport::default();
    for fixture_dir in fixture_dirs {
        let path = fixture_dir.join("report.yaml");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
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
        let mut html_paths = std::fs::read_dir(fixture_dir)
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
            let html = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
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

pub fn load_submission(fixture_dirs: &[PathBuf]) -> anyhow::Result<String> {
    let mut submissions = Vec::new();
    for fixture_dir in fixture_dirs {
        let path = fixture_dir.join("submission.txt");
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        submissions.push(content);
    }
    if submissions.is_empty() {
        anyhow::bail!("no submission files found in fixture directories");
    }
    Ok(submissions.join("\n"))
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
    if let Some(filing_receipt) = &execution.filing_receipt {
        write_json(
            &repo_root.join("artifacts/filing/filing.manifest.v1.json"),
            filing_receipt,
        )?;
    }
    Ok(())
}

pub fn assert_scenario_outcome(
    scenario: &ScenarioRecord,
    execution: &ScenarioExecution,
) -> anyhow::Result<()> {
    match scenario.ac_id.as_deref() {
        Some("AC-XK-SEC-INLINE-001") => {
            let validation_run = execution
                .validation_run
                .as_ref()
                .context("missing validation run for inline restriction scenario")?;
            ensure_report_contains_rule(validation_run, "SEC.INLINE.NO_IX_FRACTION")
        }
        Some("AC-XK-TAXONOMY-001") => {
            ensure_taxonomy_resolution_succeeds(execution)?;
            ensure_taxonomy_resolution_resolves_at_least(execution, 1)
        }
        Some("AC-XK-TAXONOMY-002") => {
            let validation_run = execution
                .validation_run
                .as_ref()
                .context("missing validation run for taxonomy scenario")?;
            ensure_report_contains_rule(validation_run, "SEC.TAXONOMY.SAME_YEAR")
        }
        Some("AC-XK-DUPLICATES-001") => {
            let validation_run = execution
                .validation_run
                .as_ref()
                .context("missing validation run for duplicate facts scenario")?;
            ensure_report_does_not_contain_rule(validation_run, "XBRL.DUPLICATE_FACT.INCONSISTENT")
        }
        Some("AC-XK-SEC-REQUIRED-001") => {
            let validation_run = execution
                .validation_run
                .as_ref()
                .context("missing validation run for required facts scenario")?;
            ensure_report_contains_rule(validation_run, "SEC.REQUIRED_FACT.DEI_ENTITYREGISTRANTNAME")
        }
        Some("AC-XK-SEC-REQUIRED-002") => {
            ensure_report_has_no_error_findings(execution)
        }
        Some("AC-XK-MANIFEST-001") => {
            if execution.filing_receipt.is_none() {
                anyhow::bail!("filing manifest receipt was not emitted");
            }
            Ok(())
        }
        Some("AC-XK-WORKFLOW-001") => {
            if execution.feature_grid.is_none() {
                anyhow::bail!("feature grid was not compiled");
            }
            Ok(())
        }
        Some("AC-XK-IXDS-001") => {
            ensure_ixds_member_count(execution, 1)?;
            ensure_report_fact_count(execution, 14)?;
            ensure_report_concept_set(execution, &[
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
            ])
        }
        Some("AC-XK-IXDS-002") => {
            ensure_ixds_member_count(execution, 2)?;
            ensure_report_fact_count(execution, 14)?;
            ensure_report_concept_set(execution, &[
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
            ])
        }
        // Scenarios without an AC ID are BDD-style scenarios that handle
        // assertions via step definitions rather than scenario-level checks
        None => Ok(()),
        _ => anyhow::bail!(
            "no scenario assertions implemented for {}",
            scenario.scenario_id
        ),
    }
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
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let fixture: EntryPointsFixture = serde_yaml::from_str(&content)
            .with_context(|| format!("parsing {}", path.display()))?;
        entry_points.extend(fixture.entry_points);
    }
    Ok(entry_points)
}

fn write_json(path: &Path, value: &impl serde::Serialize) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let bytes = serde_json::to_vec_pretty(value).context("serializing json")?;
    std::fs::write(path, bytes).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}
