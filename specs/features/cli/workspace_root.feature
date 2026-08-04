@REQ-XK-CLI
@layer.cli
@suite.synthetic
Feature: CLI workspace-root discovery

  @alpha-active
  @AC-XK-CLI-002
  @SCN-XK-CLI-002
  @speed.fast
  Scenario: Report malformed workspace-root input
    Given a malformed CLI manifest directory is supplied
    When I derive the CLI workspace root
    Then the CLI workspace root derivation should fail with path context
