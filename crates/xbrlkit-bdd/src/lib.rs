//! Minimal BDD runner for the active alpha scenarios.

use anyhow::Context;
use receipt_types::{Receipt, RunResult};
use scenario_contract::{FeatureGrid, ScenarioRecord};
use std::collections::BTreeMap;
use std::path::Path;
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
    let receipt = run_selected_scenarios(repo_root, grid, &selected, &parsed_by_id, tag)?;

    Ok(BddRun { selected, receipt })
}

fn run_selected_scenarios(
    repo_root: &Path,
    grid: &FeatureGrid,
    selected: &[ScenarioRecord],
    parsed_by_id: &BTreeMap<String, ParsedScenario>,
    tag: &str,
) -> anyhow::Result<Receipt> {
    let mut receipt = Receipt::new("scenario.run", tag, RunResult::Success);
    for scenario in selected {
        let parsed = parsed_by_id
            .get(&scenario.scenario_id)
            .with_context(|| format!("missing parsed feature for {}", scenario.scenario_id))?;
        let mut world = World::new(repo_root.to_path_buf(), grid.clone());
        run_scenario(&mut world, scenario, &parsed.steps)
            .with_context(|| format!("running scenario {}", scenario.scenario_id))?;
        receipt
            .notes
            .push(format!("{} passed", scenario.scenario_id));
    }

    Ok(receipt)
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
    let mut background_steps = Vec::<Step>::new();
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
                steps: background_steps.clone(),
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
            } else {
                background_steps.push(Step {
                    text: step_text,
                    table: Vec::new(),
                });
            }
            continue;
        }
        if line.starts_with('|') {
            if let Some(scenario) = &mut current {
                if let Some(step) = scenario.steps.last_mut() {
                    step.table.push(parse_table_row(line));
                }
            } else if let Some(step) = background_steps.last_mut() {
                step.table.push(parse_table_row(line));
            }
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
    use super::{ParsedScenario, parse_feature_file, parse_table_row, run_selected_scenarios};
    use anyhow::ensure;
    use scenario_contract::{FeatureGrid, ScenarioRecord};
    use std::collections::BTreeMap;
    use std::path::Path;
    use xbrlkit_bdd_steps::Step;

    #[test]
    fn parses_active_scenario_tags_and_steps() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .expect("workspace root")
            .join("specs/features/inline/ixds_assembly.feature");
        let scenarios = parse_feature_file(&path).expect("feature file should parse");

        assert!(
            scenarios
                .iter()
                .any(|scenario| scenario.tags.iter().any(|tag| tag == "@alpha-active"))
        );
        assert!(scenarios.iter().any(|scenario| {
            scenario
                .steps
                .iter()
                .any(|step| step.text == "I validate the filing")
        }));
    }

    #[test]
    fn parses_table_rows() {
        assert_eq!(
            parse_table_row("| dei:DocumentType |"),
            vec!["dei:DocumentType".to_string()]
        );
    }

    #[test]
    fn includes_background_steps_in_each_scenario() -> anyhow::Result<()> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .ok_or_else(|| anyhow::anyhow!("workspace root"))?
            .join("specs/features/performance/streaming_parser.feature");
        let scenarios = parse_feature_file(&path)?;

        ensure!(
            scenarios.len() == 4,
            "expected four streaming scenarios, got {}",
            scenarios.len()
        );
        for scenario in &scenarios {
            ensure!(
                scenario
                    .steps
                    .first()
                    .is_some_and(|step| { step.text == "the xbrl-stream crate is available" }),
                "background step missing from {}",
                scenario.scenario_id
            );
        }
        Ok(())
    }

    #[test]
    fn does_not_reuse_world_state_between_selected_scenarios() -> anyhow::Result<()> {
        let first = test_scenario("SCN-XK-TEST-001");
        let second = test_scenario("SCN-XK-TEST-002");
        let first_id = first.scenario_id.clone();
        let second_id = second.scenario_id.clone();
        let grid = FeatureGrid {
            scenarios: vec![first.clone(), second.clone()],
        };
        let parsed = BTreeMap::from([
            (
                first.scenario_id.clone(),
                ParsedScenario {
                    scenario_id: first.scenario_id.clone(),
                    tags: Vec::new(),
                    steps: vec![
                        Step {
                            text: "a validation report receipt".to_string(),
                            table: Vec::new(),
                        },
                        Step {
                            text: "I package the receipt for cockpit".to_string(),
                            table: Vec::new(),
                        },
                    ],
                },
            ),
            (
                second.scenario_id.clone(),
                ParsedScenario {
                    scenario_id: second.scenario_id.clone(),
                    tags: Vec::new(),
                    steps: vec![Step {
                        text: "the sensor report is emitted".to_string(),
                        table: Vec::new(),
                    }],
                },
            ),
        ]);

        let result =
            run_selected_scenarios(Path::new("."), &grid, &[first, second], &parsed, "@test");
        let error = result
            .err()
            .ok_or_else(|| anyhow::anyhow!("the second scenario unexpectedly passed"))?
            .to_string();
        ensure!(
            error.contains(&second_id),
            "expected the isolated failure to identify {second_id}, got: {error}"
        );
        ensure!(
            !error.contains(&first_id),
            "the first scenario failed before the isolation check: {error}"
        );
        Ok(())
    }

    fn test_scenario(scenario_id: &str) -> ScenarioRecord {
        ScenarioRecord {
            scenario_id: scenario_id.to_string(),
            feature_file: "test.feature".to_string(),
            sidecar_file: "test.meta.yaml".to_string(),
            layer: "test".to_string(),
            module: "test".to_string(),
            ..ScenarioRecord::default()
        }
    }
}
