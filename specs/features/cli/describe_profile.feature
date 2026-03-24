@REQ-XK-CLI
@layer.cli
@suite.synthetic
Feature: CLI describe-profile command

  @alpha-active
  @AC-XK-CLI-001
  @SCN-XK-CLI-001
  @speed.fast
  Scenario: Output profile as JSON
    Given a SEC profile is configured
    When I run describe-profile --json
    Then the output is valid JSON
    And the profile contains required fields
