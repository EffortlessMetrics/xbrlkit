@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Fixture cache lifecycle

  @AC-XK-WORKFLOW-005
  @SCN-XK-WORKFLOW-008
  @speed.fast
  @alpha-active
  Scenario: Explicit invalidation refreshes rewritten fixture content
    Given a temporary fixture report with value "1"
    When the temporary fixture report is rewritten with value "2" and the cache is invalidated
    Then the temporary fixture report loads with its rewritten value
