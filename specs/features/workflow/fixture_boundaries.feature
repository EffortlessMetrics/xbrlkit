@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Fixture boundary validation

  @AC-XK-WORKFLOW-005
  @SCN-XK-WORKFLOW-007
  @speed.fast
  @alpha-active
  Scenario: Reject a fixture path that escapes the repository root
    Given the fixture directory "../outside/fixture-01"
    Then the fixture path is rejected outside the repository root
