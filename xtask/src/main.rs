//! Repo automation for xbrlkit.

mod alpha_check;
mod schema_check;

use anyhow::Context;
use clap::{Parser, Subcommand};
use receipt_types::{Receipt, RunResult};
use scenario_contract::{BundleManifest, FeatureGrid, ImpactReport, ScenarioRecord};
use scenario_runner::{assert_scenario_outcome, execute_scenario, write_execution_receipts};
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
    let scenarios = grid.select_by_selector(selector);
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
    let scenarios = grid.select_by_selector(ac_id);
    if scenarios.is_empty() {
        anyhow::bail!("test-ac: selector matched no scenarios: {ac_id}");
    }

    let mut scenario_receipt = Receipt::new("scenario.run", ac_id, RunResult::Success);
    for scenario in &scenarios {
        let execution = execute_scenario(&repo_root(), scenario)?;
        write_execution_receipts(&repo_root(), &execution)?;
        assert_scenario_outcome(scenario, &execution)?;
        scenario_receipt
            .notes
            .push(format!("{} passed", scenario.scenario_id));
    }

    let receipt_path = repo_root().join("artifacts/runs/scenario.run.v1.json");
    write_json(&receipt_path, &scenario_receipt)?;
    println!(
        "test-ac: executed {} scenario(s) for {}",
        scenarios.len(),
        ac_id
    );
    Ok(())
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
        CargoMetadataPackage, normalize_repo_path, package_is_publishable, scenario_impacted,
    };
    use scenario_contract::FeatureGrid;

    fn scenario_record() -> scenario_contract::ScenarioRecord {
        scenario_contract::ScenarioRecord {
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
        }
    }

    #[test]
    fn selector_matching_supports_ids_and_tags() {
        let grid = FeatureGrid {
            scenarios: vec![scenario_record()],
        };

        assert_eq!(grid.select_by_selector("AC-XK-WORKFLOW-002").len(), 1);
        assert_eq!(grid.select_by_selector("SCN-XK-WORKFLOW-002").len(), 1);
        assert_eq!(grid.select_by_selector("@AC-XK-WORKFLOW-002").len(), 1);
        assert_eq!(grid.select_by_selector("@SCN-XK-WORKFLOW-002").len(), 1);
        assert!(grid.select_by_selector("AC-XK-DOES-NOT-EXIST").is_empty());
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
