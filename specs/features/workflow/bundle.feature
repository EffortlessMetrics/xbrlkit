@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Bundle

  @alpha-active
  @AC-XK-WORKFLOW-002
  @SCN-XK-WORKFLOW-002
  @speed.fast
  Scenario: Bundle an AC into a bounded context packet
    Given the feature grid is compiled
    When I bundle the selector "AC-XK-IXDS-002"
    Then the bundle manifest lists scenario "SCN-XK-IXDS-002"

  @alpha-active
  @AC-XK-WORKFLOW-002
  @SCN-XK-WORKFLOW-004
  @speed.fast
  Scenario: Reject a selector that matches no scenarios
    Given the feature grid is compiled
    When I bundle the selector "AC-XK-DOES-NOT-EXIST"
    Then bundling fails because no scenario matches
