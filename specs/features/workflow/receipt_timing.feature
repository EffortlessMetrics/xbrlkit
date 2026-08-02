@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Scenario receipt timing

  @AC-XK-WORKFLOW-005
  @SCN-XK-WORKFLOW-007
  @speed.fast
  @alpha-active
  Scenario: Record execution duration on a scenario receipt
    Given a fresh scenario run receipt
    When I record 42 milliseconds of execution
    Then the scenario run receipt reports 42 milliseconds
