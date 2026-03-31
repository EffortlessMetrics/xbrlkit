use anyhow::{Context, bail};
use chrono::Utc;
use serde::Serialize;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

const ACTIVE_ALPHA_ACS: &[&str] = &[
    "AC-XK-SEC-INLINE-001",
    "AC-XK-SEC-INLINE-002",
    "AC-XK-SEC-REQUIRED-001",
    "AC-XK-SEC-REQUIRED-002",
    // AC-XK-SEC-DECIMAL-001/002 require BDD step handlers (tested via bdd @alpha-candidate)
    "AC-XK-TAXONOMY-001",
    "AC-XK-TAXONOMY-002",
    "AC-XK-DUPLICATES-001",
    "AC-XK-IXDS-001",
    "AC-XK-IXDS-002",
    "AC-XK-EXPORT-001",
    // Streaming parser ACs - Wave 4
    "AC-XK-STREAM-001",
    "AC-XK-STREAM-002",
    "AC-XK-STREAM-003",
    "AC-XK-STREAM-004",
    // AC-XK-CONTEXT-001..004 require BDD step handlers and proper fixtures (tracked separately)
    // AC-XK-WORKFLOW-002/003 and AC-XK-MANIFEST-001 tested via @alpha-active BDD tag (no test-ac)
];

/// Summary of a single alpha-check step.
#[derive(Debug, Clone, Serialize)]
struct StepSummary {
    name: String,
    status: String,
    duration_ms: u64,
}

/// Machine-readable alpha-check summary.
#[derive(Debug, Clone, Serialize)]
struct AlphaCheckSummary {
    kind: String,
    result: String,
    steps: Vec<StepSummary>,
    scenarios_count: usize,
    timestamp: String,
}

impl AlphaCheckSummary {
    fn new() -> Self {
        Self {
            kind: "alpha-check.summary.v1".to_string(),
            result: "success".to_string(),
            steps: Vec::new(),
            scenarios_count: 0,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

/// Tracks execution of steps and builds a summary.
struct AlphaCheckRunner {
    summary: AlphaCheckSummary,
    scenarios_count: usize,
    failed: bool,
}

impl AlphaCheckRunner {
    fn new() -> Self {
        Self {
            summary: AlphaCheckSummary::new(),
            scenarios_count: 0,
            failed: false,
        }
    }

    fn run_step<F>(&mut self, name: &str, f: F) -> anyhow::Result<()>
    where
        F: FnOnce() -> anyhow::Result<()>,
    {
        let start = Instant::now();
        let result = f();
        let duration_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);

        let status = if result.is_ok() {
            "passed".to_string()
        } else {
            self.failed = true;
            "failed".to_string()
        };

        self.summary.steps.push(StepSummary {
            name: name.to_string(),
            status,
            duration_ms,
        });

        result
    }

    fn run_step_with_count<F>(&mut self, name: &str, f: F) -> anyhow::Result<usize>
    where
        F: FnOnce() -> anyhow::Result<usize>,
    {
        let start = Instant::now();
        let result = f();
        let duration_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);

        let status = if result.is_ok() {
            "passed".to_string()
        } else {
            self.failed = true;
            "failed".to_string()
        };

        self.summary.steps.push(StepSummary {
            name: name.to_string(),
            status,
            duration_ms,
        });

        result
    }

    fn add_scenarios(&mut self, count: usize) {
        self.scenarios_count += count;
    }

    fn finalize(mut self) -> AlphaCheckSummary {
        self.summary.result = if self.failed {
            "failure".to_string()
        } else {
            "success".to_string()
        };
        self.summary.scenarios_count = self.scenarios_count;
        self.summary
    }
}

pub(super) fn run() -> anyhow::Result<()> {
    let mut runner = AlphaCheckRunner::new();

    runner.run_step("doctor", super::doctor)?;

    let grid = super::load_grid()?;
    runner.run_step("feature-grid", || {
        super::write_json(
            &super::repo_root().join("artifacts/feature.grid.v1.json"),
            &grid,
        )
    })?;

    runner.run_step("schema-check", super::schema_check::run)?;
    for ac_id in ACTIVE_ALPHA_ACS {
        let step_name = format!("test-ac:{ac_id}");
        runner.run_step(&step_name, || super::test_ac(ac_id))?;
    }

    let bdd_count =
        runner.run_step_with_count("bdd:@alpha-active", || run_bdd_count("@alpha-active"))?;
    runner.add_scenarios(bdd_count);

    runner.run_step("golden:feature-grid", || {
        compare_file_to_golden(
            &super::repo_root().join("artifacts/feature.grid.v1.json"),
            &super::repo_root().join("tests/goldens/feature.grid.v1.json"),
        )
    })?;

    runner.run_step("golden:taxonomy-resolve", || {
        compare_file_to_golden(
            &super::repo_root().join("artifacts/taxonomy/taxonomy.resolve.v1.json"),
            &super::repo_root().join("tests/goldens/taxonomy.resolve.same-year.json"),
        )
    })?;

    runner.run_step("cli:describe-profile", || {
        run_cli_command(
            &["describe-profile", "--profile", "sec/efm-77/opco", "--json"],
            0,
        )
        .map(|_| ())
    })?;

    runner.run_step("cli:validate-fixture-success", || {
        run_cli_command(
            &[
                "validate-fixture",
                "--profile",
                "sec/efm-77/opco",
                "--json",
                "fixtures/synthetic/inline/ixds-single-file-01/member-a.html",
            ],
            0,
        )
        .map(|_| ())
    })?;

    let inline_failure = run_cli_command(
        &[
            "validate-fixture",
            "--profile",
            "sec/efm-77/opco",
            "--json",
            "fixtures/synthetic/sec/inline/ix-fraction-01/member-a.html",
        ],
        1,
    )?;

    runner.run_step("cli:validate-fixture-failure", || {
        compare_output_to_golden(
            &inline_failure,
            &super::repo_root()
                .join("tests/goldens/validation.report.sec-inline-no-ix-fraction.json"),
        )
    })?;

    let summary = runner.finalize();
    let summary_path = super::repo_root().join("artifacts/alpha-check.summary.v1.json");
    super::write_json(&summary_path, &summary)?;

    if summary.result == "failure" {
        anyhow::bail!("alpha-check: failed");
    }

    println!("alpha-check: active alpha gate passed");
    Ok(())
}

/// Runs BDD scenarios and returns the count of selected scenarios.
fn run_bdd_count(tag: &str) -> anyhow::Result<usize> {
    let grid = super::load_grid()?;
    let path = super::repo_root().join("artifacts/runs/scenario.run.v1.json");
    let run = match xbrlkit_bdd::run(&super::repo_root(), &grid, tag) {
        Ok(run) => run,
        Err(error) => {
            use receipt_types::{Receipt, RunResult};
            let mut receipt = Receipt::new("scenario.run", tag, RunResult::Error);
            receipt.notes.push(error.to_string());
            super::write_json(&path, &receipt)?;
            return Err(error);
        }
    };
    super::write_json(&path, &run.receipt)?;
    println!("bdd: selected {} scenarios for {}", run.selected.len(), tag);
    Ok(run.selected.len())
}

fn run_cli_command(args: &[&str], expected_exit_code: i32) -> anyhow::Result<String> {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "-p", "xbrlkit-cli", "--"])
        .args(args)
        .current_dir(super::repo_root())
        .output()
        .with_context(|| format!("running cargo xbrlkit-cli {args:?}"))?;

    let actual_code = output.status.code().unwrap_or(-1);
    if actual_code != expected_exit_code {
        bail!(
            "xbrlkit-cli {:?} exited with {}, expected {}\nstdout:\n{}\nstderr:\n{}",
            args,
            actual_code,
            expected_exit_code,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).context("decoding xbrlkit-cli stdout")
}

fn compare_output_to_golden(output: &str, golden_path: &Path) -> anyhow::Result<()> {
    let golden = std::fs::read_to_string(golden_path)
        .with_context(|| format!("reading {}", golden_path.display()))?;
    if normalize(output) == normalize(&golden) {
        Ok(())
    } else {
        bail!("output does not match golden {}", golden_path.display())
    }
}

fn compare_file_to_golden(actual_path: &Path, golden_path: &Path) -> anyhow::Result<()> {
    let actual = std::fs::read_to_string(actual_path)
        .with_context(|| format!("reading {}", actual_path.display()))?;
    compare_output_to_golden(&actual, golden_path)
}

fn normalize(value: &str) -> String {
    value.replace("\r\n", "\n").trim().to_string()
}
