@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Fixture path security

  @AC-XK-WORKFLOW-007
  @SCN-XK-WORKFLOW-007
  @speed.fast
  @alpha-candidate
  Scenario: Reject fixture parent-directory traversal
    Given the fixture directory "../outside"
    When I validate the declared fixture paths
    Then the fixture path is rejected outside the repository root
