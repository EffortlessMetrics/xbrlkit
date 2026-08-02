//! Repo automation for xbrlkit.

mod alpha_check;
mod schema_check;

use anyhow::Context;
use clap::{Parser, Subcommand};
use receipt_types::{Receipt, RunResult};
use scenario_contract::{BundleManifest, FeatureGrid, ImpactReport, ScenarioRecord};
use scenario_runner::{assert_scenario_outcome, execute_scenario, write_execution_receipts};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

#[derive(Debug, Parser)]
#[command(name = "xtask")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Sanity-check required repo directories.
    Doctor,
    /// Compile the feature grid from sidecars.
    FeatureGrid,
    /// Create a bounded context bundle for a selector.
    Bundle { selector: String },
    /// Estimate impacted scenarios from changed paths.
    Impact {
        #[arg(long = "changed", required = true)]
        changed: Vec<String>,
    },
    /// Focused AC helper.
    TestAc { ac_id: String },
    /// Validate emitted JSON outputs against the checked-in schemas.
    SchemaCheck,
    /// Run the alpha upload gate.
    AlphaCheck,
    /// Verify publishable crates package cleanly for crates.io.
    PackageCheck,
    /// Run the active BDD scenarios selected by tag.
    Bdd {
        #[arg(long)]
        tags: Option<String>,
    },
    /// Wrap a validation receipt into sensor.report.v1.
    CockpitPack,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Doctor => doctor(),
        Command::FeatureGrid => {
            let grid = load_grid()?;
            write_json(&repo_root().join("artifacts/feature.grid.v1.json"), &grid)?;
            println!("compiled {} scenarios", grid.scenarios.len());
            Ok(())
        }
        Command::Bundle { selector } => bundle(&selector),
        Command::Impact { changed } => impact(&changed),
        Command::TestAc { ac_id } => test_ac(&ac_id),
        Command::SchemaCheck => schema_check::run(),
        Command::AlphaCheck => alpha_check::run(),
        Command::PackageCheck => package_check(),
        Command::Bdd { tags } => bdd(tags.as_deref().unwrap_or("@alpha-active")),
        Command::CockpitPack => cockpit_pack(),
    }
}

fn repo_root() -> PathBuf {
    // Use git to detect worktree root at runtime
    // This correctly handles git worktrees where the compile-time CARGO_MANIFEST_DIR
    // points to the anchor clone, but we want artifacts in the active worktree
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        && output.status.success()
    {
        let path = String::from_utf8_lossy(&output.stdout);
        return PathBuf::from(path.trim());
    }

    // Fallback: compile-time path for non-git environments
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has a parent workspace root")
        .to_path_buf()
}

fn load_grid() -> anyhow::Result<FeatureGrid> {
    xbrlkit_feature_grid::compile(&repo_root())
}

fn doctor() -> anyhow::Result<()> {
    let root = repo_root();
    for required in [
        "contracts/schemas",
        "specs/features",
        "profiles/sec/efm-77/opco",
        "fixtures/synthetic",
    ] {
        let path = root.join(required);
        if !path.exists() {
            anyhow::bail!("missing required path: {}", path.display());
        }
    }
    println!("doctor: repo layout looks healthy");
    Ok(())
}

fn bundle(selector: &str) -> anyhow::Result<()> {
    let grid = load_grid()?;
    let scenarios = select_matching_scenarios(&grid, selector);
    if scenarios.is_empty() {
        anyhow::bail!("bundle: selector matched no scenarios: {selector}");
    }
    let manifest = BundleManifest {
        selector: selector.to_string(),
        scenarios,
    };
    let path = repo_root().join(format!("artifacts/bundles/{}.json", sanitize(selector)));
    write_json(&path, &manifest)?;
    println!("bundle: wrote {}", path.display());
    Ok(())
}

fn impact(changed: &[String]) -> anyhow::Result<()> {
    let grid = load_grid()?;
    let normalized_changed = changed
        .iter()
        .map(|path| normalize_repo_path(path))
        .collect::<Vec<_>>();
    let impacted = grid
        .scenarios
        .iter()
        .filter(|scenario| scenario_impacted(scenario, &normalized_changed))
        .map(|scenario| scenario.scenario_id.clone())
        .collect::<Vec<_>>();
    let report = ImpactReport {
        changed_paths: normalized_changed,
        impacted_scenarios: impacted,
    };
    let path = repo_root().join("artifacts/impact/impact.report.v1.json");
    write_json(&path, &report)?;
    println!("impact: wrote {}", path.display());
    Ok(())
}

fn test_ac(ac_id: &str) -> anyhow::Result<()> {
    let grid = load_grid()?;
    let scenarios = select_matching_scenarios(&grid, ac_id);
    if scenarios.is_empty() {
        anyhow::bail!("test-ac: selector matched no scenarios: {ac_id}");
    }

    let receipt_path = repo_root().join("artifacts/runs/scenario.run.v1.json");
    let result = match resolve_test_mode(&scenarios) {
        Ok(TestMode::Bdd) => run_bdd_scenarios(ac_id, &grid, &scenarios),
        Ok(TestMode::ScenarioRunner) => run_direct_scenarios(ac_id, &scenarios),
        Err(error) => Err(error),
    };
    let receipt = match result {
        Ok(receipt) => receipt,
        Err(error) => {
            let receipt = test_ac_error_receipt(ac_id, &error);
            write_json(&receipt_path, &receipt)?;
            return Err(error);
        }
    };
    write_json(&receipt_path, &receipt)?;
    println!(
        "test-ac: executed {} scenario(s) for {}",
        scenarios.len(),
        ac_id
    );
    Ok(())
}

fn test_ac_error_receipt(ac_id: &str, error: &anyhow::Error) -> Receipt {
    let mut receipt = Receipt::new("scenario.run", ac_id, RunResult::Error);
    receipt.notes.push(error.to_string());
    receipt
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestMode {
    Bdd,
    ScenarioRunner,
}

fn resolve_test_mode(scenarios: &[ScenarioRecord]) -> anyhow::Result<TestMode> {
    let first = scenarios
        .first()
        .context("test-ac: no scenarios supplied for mode resolution")?;
    let test_type = first
        .test_type
        .as_deref()
        .context("test-ac: scenario is missing declared test type")?;
    if first.test_tag.is_none() {
        anyhow::bail!(
            "test-ac: scenario {} is missing declared test tag",
            first.scenario_id
        );
    }

    let mode = match test_type {
        "bdd" => TestMode::Bdd,
        "direct" | "scenario-runner" => TestMode::ScenarioRunner,
        other => anyhow::bail!(
            "test-ac: scenario {} declares unsupported test type {}; expected bdd, direct, or scenario-runner",
            first.scenario_id,
            other
        ),
    };

    for scenario in scenarios.iter().skip(1) {
        let scenario_type = scenario.test_type.as_deref().with_context(|| {
            format!(
                "test-ac: scenario {} is missing declared test type",
                scenario.scenario_id
            )
        })?;
        if scenario.test_tag.is_none() {
            anyhow::bail!(
                "test-ac: scenario {} is missing declared test tag",
                scenario.scenario_id
            );
        }
        let scenario_mode = match scenario_type {
            "bdd" => TestMode::Bdd,
            "direct" | "scenario-runner" => TestMode::ScenarioRunner,
            other => anyhow::bail!(
                "test-ac: scenario {} declares unsupported test type {}; expected bdd, direct, or scenario-runner",
                scenario.scenario_id,
                other
            ),
        };
        if scenario_mode != mode {
            anyhow::bail!(
                "test-ac: selector mixes declared test modes: {} is {:?}, {} is {:?}",
                first.scenario_id,
                mode,
                scenario.scenario_id,
                scenario_mode
            );
        }
    }
    Ok(mode)
}

fn run_direct_scenarios(ac_id: &str, scenarios: &[ScenarioRecord]) -> anyhow::Result<Receipt> {
    let mut receipt = Receipt::new("scenario.run", ac_id, RunResult::Success);
    for scenario in scenarios {
        let execution = execute_scenario(&repo_root(), scenario)?;
        write_execution_receipts(&repo_root(), &execution)?;
        assert_scenario_outcome(scenario, &execution)?;
        receipt
            .notes
            .push(format!("{} passed", scenario.scenario_id));
    }
    Ok(receipt)
}

fn bdd_groups(scenarios: &[ScenarioRecord]) -> anyhow::Result<BTreeMap<String, Vec<String>>> {
    let mut groups = BTreeMap::<String, Vec<String>>::new();
    for scenario in scenarios {
        let tag = scenario.test_tag.as_ref().with_context(|| {
            format!("scenario {} is missing BDD test tag", scenario.scenario_id)
        })?;
        groups
            .entry(tag.clone())
            .or_default()
            .push(scenario.scenario_id.clone());
    }
    for expected_ids in groups.values_mut() {
        expected_ids.sort();
    }
    Ok(groups)
}

fn run_bdd_scenarios(
    ac_id: &str,
    grid: &FeatureGrid,
    scenarios: &[ScenarioRecord],
) -> anyhow::Result<Receipt> {
    let mut receipt = Receipt::new("scenario.run", ac_id, RunResult::Success);
    for (tag, expected_ids) in bdd_groups(scenarios)? {
        let run = xbrlkit_bdd::run(&repo_root(), grid, &tag)?;
        let mut selected_ids = run
            .selected
            .iter()
            .map(|scenario| scenario.scenario_id.clone())
            .collect::<Vec<_>>();
        selected_ids.sort();
        if selected_ids != expected_ids {
            anyhow::bail!(
                "test-ac: BDD tag {tag} selected {selected_ids:?}, expected {expected_ids:?}"
            );
        }
        receipt.notes.extend(
            run.receipt
                .notes
                .into_iter()
                .map(|note| format!("{tag}: {note}")),
        );
    }
    Ok(receipt)
}

fn bdd(tag: &str) -> anyhow::Result<()> {
    let grid = load_grid()?;
    let path = repo_root().join("artifacts/runs/scenario.run.v1.json");
    let run = match xbrlkit_bdd::run(&repo_root(), &grid, tag) {
        Ok(run) => run,
        Err(error) => {
            let mut receipt = Receipt::new("scenario.run", tag, RunResult::Error);
            receipt.notes.push(error.to_string());
            write_json(&path, &receipt)?;
            return Err(error);
        }
    };
    write_json(&path, &run.receipt)?;
    println!("bdd: selected {} scenarios for {}", run.selected.len(), tag);
    Ok(())
}

fn cockpit_pack() -> anyhow::Result<()> {
    let receipt = Receipt::new(
        "validation.report",
        "workspace-validation",
        RunResult::Success,
    );
    let value = cockpit_export::to_sensor_report("xbrlkit", &receipt);
    let path = repo_root().join("artifacts/cockpit/sensor.report.v1.json");
    write_json(&path, &value)?;
    println!("cockpit-pack: wrote {}", path.display());
    Ok(())
}

fn package_check() -> anyhow::Result<()> {
    let packages = publishable_packages()?;
    for package in &packages {
        run_cargo_package(package)?;
    }
    println!("package-check: packaged {} crate(s)", packages.len());
    Ok(())
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

fn publishable_packages() -> anyhow::Result<Vec<String>> {
    let output = ProcessCommand::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(repo_root())
        .output()
        .context("running cargo metadata for package-check")?;
    if !output.status.success() {
        anyhow::bail!(
            "package-check: cargo metadata failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let metadata: CargoMetadata =
        serde_json::from_slice(&output.stdout).context("parsing cargo metadata output")?;
    let mut packages = metadata
        .packages
        .into_iter()
        .filter(package_is_publishable)
        .map(|package| package.name)
        .collect::<Vec<_>>();
    packages.sort();
    Ok(packages)
}

fn package_is_publishable(package: &CargoMetadataPackage) -> bool {
    package
        .publish
        .as_ref()
        .is_none_or(|registries| !registries.is_empty())
}

fn run_cargo_package(package: &str) -> anyhow::Result<()> {
    let output = ProcessCommand::new("cargo")
        .args([
            "package",
            "-p",
            package,
            "--allow-dirty",
            "--locked",
            "--list",
        ])
        .current_dir(repo_root())
        .output()
        .with_context(|| format!("packaging {package}"))?;
    if output.status.success() {
        Ok(())
    } else {
        anyhow::bail!(
            "package-check: cargo package failed for {package}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[derive(Debug, serde::Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoMetadataPackage>,
}

#[derive(Debug, serde::Deserialize)]
struct CargoMetadataPackage {
    name: String,
    publish: Option<Vec<String>>,
}

fn sanitize(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn select_matching_scenarios(grid: &FeatureGrid, selector: &str) -> Vec<ScenarioRecord> {
    grid.scenarios
        .iter()
        .filter(|scenario| selector_matches(scenario, selector))
        .cloned()
        .collect()
}

fn selector_matches(scenario: &ScenarioRecord, selector: &str) -> bool {
    scenario.scenario_id == selector
        || scenario.ac_id.as_deref() == Some(selector)
        || scenario.req_id.as_deref() == Some(selector)
        || format!("@{}", scenario.scenario_id) == selector
        || scenario
            .ac_id
            .as_ref()
            .is_some_and(|ac| format!("@{ac}") == selector)
}

fn normalize_repo_path(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}

fn scenario_impacted(scenario: &ScenarioRecord, changed: &[String]) -> bool {
    let allowed_edit_roots = scenario
        .allowed_edit_roots
        .iter()
        .map(|root| normalize_repo_path(root))
        .collect::<Vec<_>>();

    changed.iter().any(|changed_path| {
        allowed_edit_roots
            .iter()
            .any(|root| changed_path.starts_with(root))
            || scenario
                .crates
                .iter()
                .any(|crate_name| changed_path.contains(crate_name))
    })
}

#[cfg(test)]
mod tests {
    use super::{
        CargoMetadataPackage, TestMode, bdd_groups, normalize_repo_path, package_is_publishable,
        resolve_test_mode, scenario_impacted, select_matching_scenarios,
    };
    use scenario_contract::{FeatureGrid, ScenarioRecord};

    fn scenario_record() -> ScenarioRecord {
        ScenarioRecord {
            scenario_id: "SCN-XK-WORKFLOW-002".to_string(),
            ac_id: Some("AC-XK-WORKFLOW-002".to_string()),
            req_id: Some("REQ-XK-WORKFLOW".to_string()),
            feature_file: "specs/features/workflow/bundle.feature".to_string(),
            sidecar_file: "specs/features/workflow/bundle.meta.yaml".to_string(),
            layer: "workflow".to_string(),
            module: "bundle".to_string(),
            crates: vec!["xtask".to_string()],
            fixtures: Vec::new(),
            profile_pack: None,
            receipts: vec!["bundle.manifest.v1".to_string()],
            allowed_edit_roots: vec!["specs/features/workflow".to_string(), "xtask".to_string()],
            suite: Some("synthetic".to_string()),
            speed: Some("fast".to_string()),
            test_type: Some("bdd".to_string()),
            test_tag: Some("@AC-XK-WORKFLOW-002".to_string()),
        }
    }

    #[test]
    fn test_ac_requires_declared_mode_and_tag() -> anyhow::Result<()> {
        let mut scenario = scenario_record();
        scenario.test_type = None;
        let Err(error) = resolve_test_mode(&[scenario]) else {
            return Err(anyhow::anyhow!("missing test type unexpectedly resolved"));
        };
        if !error.to_string().contains("missing declared test type") {
            return Err(anyhow::anyhow!("unexpected error: {error}"));
        }

        let mut scenario = scenario_record();
        scenario.test_tag = None;
        let Err(error) = resolve_test_mode(&[scenario]) else {
            return Err(anyhow::anyhow!("missing test tag unexpectedly resolved"));
        };
        if !error.to_string().contains("missing declared test tag") {
            return Err(anyhow::anyhow!("unexpected error: {error}"));
        }
        Ok(())
    }

    #[test]
    fn test_ac_mode_failures_have_error_receipts() -> anyhow::Result<()> {
        let error = anyhow::anyhow!("missing declared test type");
        let receipt = super::test_ac_error_receipt("AC-XK-MISSING", &error);
        if receipt.result != super::RunResult::Error
            || receipt.subject != "AC-XK-MISSING"
            || receipt.notes != vec!["missing declared test type".to_string()]
        {
            return Err(anyhow::anyhow!(
                "mode failure receipt was not deterministic"
            ));
        }
        Ok(())
    }

    #[test]
    fn test_ac_rejects_mixed_declared_modes() -> anyhow::Result<()> {
        let mut direct = scenario_record();
        direct.scenario_id = "SCN-XK-WORKFLOW-003".to_string();
        direct.test_type = Some("scenario-runner".to_string());
        direct.test_tag = Some("@SCN-XK-WORKFLOW-003".to_string());
        if resolve_test_mode(&[direct.clone()])? != TestMode::ScenarioRunner {
            return Err(anyhow::anyhow!(
                "direct declaration did not resolve to scenario-runner mode"
            ));
        }
        let Err(error) = resolve_test_mode(&[scenario_record(), direct]) else {
            return Err(anyhow::anyhow!("mixed modes unexpectedly resolved"));
        };
        if !error.to_string().contains("mixes declared test modes") {
            return Err(anyhow::anyhow!("unexpected error: {error}"));
        }
        Ok(())
    }

    #[test]
    fn bdd_groups_are_exact_and_deterministic() -> anyhow::Result<()> {
        let mut second = scenario_record();
        second.scenario_id = "SCN-XK-WORKFLOW-001".to_string();
        second.test_tag = Some("@AC-XK-WORKFLOW-001".to_string());
        let groups = bdd_groups(&[scenario_record(), second])?;
        if groups.get("@AC-XK-WORKFLOW-002") != Some(&vec!["SCN-XK-WORKFLOW-002".to_string()]) {
            return Err(anyhow::anyhow!("unexpected workflow-002 BDD group"));
        }
        if groups.get("@AC-XK-WORKFLOW-001") != Some(&vec!["SCN-XK-WORKFLOW-001".to_string()]) {
            return Err(anyhow::anyhow!("unexpected workflow-001 BDD group"));
        }
        if resolve_test_mode(&[scenario_record()])? != TestMode::Bdd {
            return Err(anyhow::anyhow!(
                "BDD declaration did not resolve to BDD mode"
            ));
        }
        Ok(())
    }

    #[test]
    fn selector_matching_supports_ids_and_tags() {
        let grid = FeatureGrid {
            scenarios: vec![scenario_record()],
        };

        assert_eq!(
            select_matching_scenarios(&grid, "AC-XK-WORKFLOW-002").len(),
            1
        );
        assert_eq!(
            select_matching_scenarios(&grid, "SCN-XK-WORKFLOW-002").len(),
            1
        );
        assert_eq!(
            select_matching_scenarios(&grid, "@AC-XK-WORKFLOW-002").len(),
            1
        );
        assert_eq!(
            select_matching_scenarios(&grid, "@SCN-XK-WORKFLOW-002").len(),
            1
        );
        assert!(select_matching_scenarios(&grid, "AC-XK-DOES-NOT-EXIST").is_empty());
    }

    #[test]
    fn impact_normalizes_windows_paths_before_matching() {
        let scenario = scenario_record();
        let changed = vec![normalize_repo_path(
            "specs\\features\\workflow\\bundle.feature",
        )];

        assert!(scenario_impacted(&scenario, &changed));
    }

    #[test]
    fn package_check_skips_workspace_only_crates() {
        assert!(package_is_publishable(&CargoMetadataPackage {
            name: "xbrlkit-core".to_string(),
            publish: None,
        }));
        assert!(!package_is_publishable(&CargoMetadataPackage {
            name: "xtask".to_string(),
            publish: Some(Vec::new()),
        }));
        assert!(package_is_publishable(&CargoMetadataPackage {
            name: "custom-registry-crate".to_string(),
            publish: Some(vec!["crates-io".to_string()]),
        }));
    }
}
