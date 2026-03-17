//! CLI edge for xbrlkit.

use anyhow::Context;
use clap::{Parser, Subcommand};
use sec_profile_types::{ProfilePack, load_profile_from_workspace};
use std::path::{Path, PathBuf};
use validation_run::validate_html_members;
use xbrl_report_types::ValidationFinding;

#[derive(Debug, Parser)]
#[command(name = "xbrlkit")]
#[command(about = "Scenario-driven XBRL / EDGAR workspace CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate one or more HTML fixtures using the selected SEC profile pack.
    ValidateFixture {
        #[arg(long)]
        profile: String,
        #[arg(long)]
        json: bool,
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
    /// Print a quick profile summary.
    DescribeProfile {
        #[arg(long)]
        profile: String,
        #[arg(long)]
        json: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let exit_code = match cli.command {
        Command::ValidateFixture {
            profile,
            json,
            files,
        } => {
            let profile = load_profile(&profile)?;
            let mut owned = Vec::<(String, String)>::new();
            for path in files {
                let html = corpus_fs::read_to_string(&path)?;
                let name =
                    if let Some(file_name) = path.file_name().and_then(|value| value.to_str()) {
                        file_name.to_string()
                    } else {
                        path.to_string_lossy().into_owned()
                    };
                owned.push((name, html));
            }
            let members = owned
                .iter()
                .map(|(name, html)| (name.as_str(), html.as_str()))
                .collect::<Vec<_>>();
            let run = validate_html_members(&members, &profile);
            if json {
                let (json_output, _) = export_run::export_json(&run.report);
                println!("{}", render_json::format_json(&json_output));
            } else {
                print_validation_summary(&profile, &run);
            }
            exit_code_from_findings(&run.report.findings)
        }
        Command::DescribeProfile { profile, json } => {
            let profile = load_profile(&profile)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&profile).context("serializing profile json")?
                );
            } else {
                println!(
                    "{}",
                    serde_yaml::to_string(&profile).context("serializing profile yaml")?
                );
            }
            0
        }
    };
    std::process::exit(exit_code)
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
}

fn load_profile(profile_id: &str) -> anyhow::Result<ProfilePack> {
    load_profile_from_workspace(workspace_root(), profile_id)
}

fn print_validation_summary(profile: &ProfilePack, run: &validation_run::ValidationRun) {
    println!("profile: {}", profile.id);
    println!("members: {}", run.report.members.len());
    println!("facts: {}", run.report.facts.len());
    if run.report.findings.is_empty() {
        println!("findings: none");
    } else {
        for finding in &run.report.findings {
            println!(
                "{} {} {}",
                finding.severity, finding.rule_id, finding.message
            );
        }
    }
}

fn exit_code_from_findings(findings: &[ValidationFinding]) -> i32 {
    i32::from(findings.iter().any(|finding| finding.severity == "error"))
}
