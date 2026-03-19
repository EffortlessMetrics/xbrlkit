use anyhow::{Context, bail};
use std::path::Path;
use std::process::Command;

const ACTIVE_ALPHA_ACS: &[&str] = &[
    "AC-XK-SEC-INLINE-001",
    "AC-XK-SEC-REQUIRED-001",
    "AC-XK-SEC-REQUIRED-002",
    "AC-XK-TAXONOMY-001",
    "AC-XK-TAXONOMY-002",
    "AC-XK-DUPLICATES-001",
    "AC-XK-IXDS-001",
    "AC-XK-IXDS-002",
    "AC-XK-EXPORT-001",
    "AC-XK-WORKFLOW-001",
    "AC-XK-MANIFEST-001",
    "AC-XK-WORKFLOW-002",
];

pub(super) fn run() -> anyhow::Result<()> {
    super::doctor()?;
    let grid = super::load_grid()?;
    super::write_json(
        &super::repo_root().join("artifacts/feature.grid.v1.json"),
        &grid,
    )?;
    super::schema_check::run()?;
    for ac_id in ACTIVE_ALPHA_ACS {
        super::test_ac(ac_id)?;
    }
    super::bdd("@alpha-active")?;

    compare_file_to_golden(
        &super::repo_root().join("artifacts/feature.grid.v1.json"),
        &super::repo_root().join("tests/goldens/feature.grid.v1.json"),
    )?;
    compare_file_to_golden(
        &super::repo_root().join("artifacts/taxonomy/taxonomy.resolve.v1.json"),
        &super::repo_root().join("tests/goldens/taxonomy.resolve.same-year.json"),
    )?;

    run_cli_command(
        &["describe-profile", "--profile", "sec/efm-77/opco", "--json"],
        0,
    )?;
    run_cli_command(
        &[
            "validate-fixture",
            "--profile",
            "sec/efm-77/opco",
            "--json",
            "fixtures/synthetic/inline/ixds-single-file-01/member-a.html",
        ],
        0,
    )?;
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
    compare_output_to_golden(
        &inline_failure,
        &super::repo_root().join("tests/goldens/validation.report.sec-inline-no-ix-fraction.json"),
    )?;

    println!("alpha-check: active alpha gate passed");
    Ok(())
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
