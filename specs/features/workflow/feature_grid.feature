@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Feature grid

  @AC-XK-WORKFLOW-001
  @SCN-XK-WORKFLOW-001
  @alpha-active
  @speed.fast
  Scenario: Compile a feature grid
    Given the profile pack "sec/efm-77/opco"
    Given the fixture "synthetic/workflow/bundle-test"
    Given the feature grid is compiled
    When the scenario executes
    Then the feature grid contains scenario "SCN-XK-IXDS-002"
