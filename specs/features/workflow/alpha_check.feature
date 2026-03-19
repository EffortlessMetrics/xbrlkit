@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Alpha check

  @AC-XK-WORKFLOW-003
  @SCN-XK-WORKFLOW-005
  @speed.fast
  Scenario: Run the alpha readiness gate
    Given the profile pack "sec/efm-77/opco"
    Given the fixture "synthetic/workflow/alpha-check-test"
    Given the active alpha scenarios are implemented
    When I run the alpha readiness gate
    Then the alpha readiness checks pass
