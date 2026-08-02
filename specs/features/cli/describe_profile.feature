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

  @AC-XK-CLI-002
  @SCN-XK-CLI-002
  @speed.fast
  Scenario: Inspect JSON serialization failures fail closed
    Given a synthetic CLI JSON serialization failure
    When I serialize the inspect-contexts and inspect-taxonomy JSON responses
    Then both inspect JSON responses fail with exit code "1"
    And the CLI error output names both JSON responses
