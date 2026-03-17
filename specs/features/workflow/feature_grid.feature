@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Feature grid

  @AC-XK-WORKFLOW-001
  @SCN-XK-WORKFLOW-001
  @speed.fast
  Scenario: Compile a feature grid
    Given the repo has feature sidecars
    When I compile the feature grid
    Then the feature grid contains scenario "SCN-XK-IXDS-002"
