//! Minimal BDD runner for the active alpha scenarios.

use anyhow::Context;
use receipt_types::{Receipt, RunResult};
use scenario_contract::{FeatureGrid, ScenarioRecord};
use std::collections::BTreeMap;
use std::path::Path;
use std::time::Instant;
use xbrlkit_bdd_steps::{Step, World, run_scenario};

#[derive(Debug, Clone)]
pub struct BddRun {
    pub selected: Vec<ScenarioRecord>,
    pub receipt: Receipt,
}

#[derive(Debug, Clone)]
struct ParsedScenario {
    scenario_id: String,
    tags: Vec<String>,
    steps: Vec<Step>,
}

pub fn run(repo_root: &Path, grid: &FeatureGrid, tag: &str) -> anyhow::Result<BddRun> {
    let parsed = parse_feature_scenarios(repo_root)?;
    let selected = select_by_tag(grid, &parsed, tag);
    if selected.is_empty() {
        anyhow::bail!("bdd: no scenarios matched {tag}");
    }

    let parsed_by_id = parsed
        .into_iter()
        .map(|scenario| (scenario.scenario_id.clone(), scenario))
        .collect::<BTreeMap<_, _>>();
    let mut world = World::new(repo_root.to_path_buf(), grid.clone());
    let mut receipt = Receipt::new("scenario.run", tag, RunResult::Success);
    let execution_start = Instant::now();
    for scenario in &selected {
        let parsed = parsed_by_id
            .get(&scenario.scenario_id)
            .with_context(|| format!("missing parsed feature for {}", scenario.scenario_id))?;
        world.profile_id = None;
        world.fixture_dirs.clear();
        world.execution = None;
        let scenario_start = Instant::now();
        run_scenario(&mut world, scenario, &parsed.steps)?;
        receipt.notes.push(format!(
            "{} passed in {} ms",
            scenario.scenario_id,
            scenario_start.elapsed().as_millis()
        ));
    }
    receipt.set_execution_duration(execution_start.elapsed());

    Ok(BddRun { selected, receipt })
}

fn select_by_tag(grid: &FeatureGrid, parsed: &[ParsedScenario], tag: &str) -> Vec<ScenarioRecord> {
    let selected_ids = parsed
        .iter()
        .filter(|scenario| scenario.tags.iter().any(|candidate| candidate == tag))
        .map(|scenario| scenario.scenario_id.as_str())
        .collect::<Vec<_>>();

    grid.scenarios
        .iter()
        .filter(|scenario| {
            selected_ids
                .iter()
                .any(|selected| *selected == scenario.scenario_id)
        })
        .cloned()
        .collect()
}

fn parse_feature_scenarios(repo_root: &Path) -> anyhow::Result<Vec<ParsedScenario>> {
    let features_root = repo_root.join("specs/features");
    let mut parsed = Vec::new();
    for entry in walkdir::WalkDir::new(&features_root)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file()
            || entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "feature")
        {
            continue;
        }
        parsed.extend(parse_feature_file(entry.path())?);
    }
    Ok(parsed)
}

fn parse_feature_file(path: &Path) -> anyhow::Result<Vec<ParsedScenario>> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut feature_tags = Vec::<String>::new();
    let mut pending_tags = Vec::<String>::new();
    let mut scenarios = Vec::<ParsedScenario>::new();
    let mut current: Option<ParsedScenario> = None;
    let mut feature_header_seen = false;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('@') {
            let tags = line
                .split_whitespace()
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            if !feature_header_seen && current.is_none() {
                feature_tags.extend(tags);
            } else {
                pending_tags.extend(tags);
            }
            continue;
        }
        if line.starts_with("Feature:") {
            feature_header_seen = true;
            continue;
        }
        if line.starts_with("Scenario:") {
            if let Some(scenario) = current.take() {
                scenarios.push(scenario);
            }
            let mut tags = feature_tags.clone();
            tags.append(&mut pending_tags);
            let scenario_id = tags
                .iter()
                .find_map(|tag| {
                    tag.strip_prefix("@SCN-")
                        .map(|suffix| format!("SCN-{suffix}"))
                })
                .with_context(|| {
                    format!("scenario in {} is missing an @SCN tag", path.display())
                })?;
            current = Some(ParsedScenario {
                scenario_id,
                tags,
                steps: Vec::new(),
            });
            continue;
        }
        if matches_step_line(line) {
            let step_text = line
                .split_once(' ')
                .map(|(_, remainder)| remainder.to_string())
                .unwrap_or_default();
            if let Some(scenario) = &mut current {
                scenario.steps.push(Step {
                    text: step_text,
                    table: Vec::new(),
                });
            }
            continue;
        }
        if line.starts_with('|')
            && let Some(scenario) = &mut current
            && let Some(step) = scenario.steps.last_mut()
        {
            step.table.push(parse_table_row(line));
        }
    }

    if let Some(scenario) = current {
        scenarios.push(scenario);
    }
    Ok(scenarios)
}

fn matches_step_line(line: &str) -> bool {
    ["Given ", "When ", "Then ", "And "]
        .iter()
        .any(|prefix| line.starts_with(prefix))
}

fn parse_table_row(line: &str) -> Vec<String> {
    line.trim_matches('|')
        .split('|')
        .map(str::trim)
        .filter(|cell| !cell.is_empty())
        .map(ToString::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_feature_file, parse_table_row, run};
    use scenario_contract::{FeatureGrid, ScenarioRecord};
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

    struct TempRoot(PathBuf);

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn temp_root() -> Result<TempRoot, String> {
        let id = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("xbrlkit-bdd-timing-{}-{id}", std::process::id()));
        fs::create_dir_all(root.join("specs/features/workflow"))
            .map_err(|error| error.to_string())?;
        Ok(TempRoot(root))
    }

    #[test]
    fn parses_active_scenario_tags_and_steps() -> Result<(), String> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .ok_or_else(|| "workspace root is missing".to_string())?
            .join("specs/features/inline/ixds_assembly.feature");
        let scenarios = parse_feature_file(&path).map_err(|error| error.to_string())?;

        if !scenarios
            .iter()
            .any(|scenario| scenario.tags.iter().any(|tag| tag == "@alpha-active"))
        {
            return Err("no active scenario tag was parsed".to_string());
        }
        if !scenarios.iter().any(|scenario| {
            scenario
                .steps
                .iter()
                .any(|step| step.text == "I validate the filing")
        }) {
            return Err("expected filing-validation step was not parsed".to_string());
        }
        Ok(())
    }

    #[test]
    fn parses_table_rows() -> Result<(), String> {
        let parsed = parse_table_row("| dei:DocumentType |");
        if parsed != vec!["dei:DocumentType".to_string()] {
            return Err(format!("unexpected parsed table row: {parsed:?}"));
        }
        Ok(())
    }

    #[test]
    fn run_records_execution_timing_and_scenario_note() -> Result<(), String> {
        let root = temp_root()?;
        let feature = "@REQ-XK-TEST\nFeature: Timing\n\n  @alpha-active @SCN-XK-TEST-TIMING-001\n  Scenario: Record timing\n    Given a fresh scenario run receipt\n    When I record 42 milliseconds of execution\n    Then the scenario run receipt reports 42 milliseconds\n";
        fs::write(
            root.0.join("specs/features/workflow/timing.feature"),
            feature,
        )
        .map_err(|error| error.to_string())?;

        let scenario_id = "SCN-XK-TEST-TIMING-001".to_string();
        let grid = FeatureGrid {
            scenarios: vec![ScenarioRecord {
                scenario_id: scenario_id.clone(),
                feature_file: "specs/features/workflow/timing.feature".to_string(),
                ..ScenarioRecord::default()
            }],
        };
        let bdd_run = run(&root.0, &grid, "@alpha-active").map_err(|error| error.to_string())?;

        if bdd_run.receipt.execution_duration_ms.is_none() {
            return Err("BDD run receipt did not record execution duration".to_string());
        }
        let note = bdd_run
            .receipt
            .notes
            .first()
            .ok_or_else(|| "BDD run receipt has no scenario timing note".to_string())?;
        if !note.starts_with(&format!("{scenario_id} passed in ")) || !note.ends_with(" ms") {
            return Err(format!("unexpected scenario timing note: {note}"));
        }
        Ok(())
    }
}
