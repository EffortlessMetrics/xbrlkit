@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Package check

  @AC-XK-WORKFLOW-004
  @SCN-XK-WORKFLOW-006
  @speed.fast
  Scenario: Verify publishable crates package for crates.io
    Given the publishable workspace crates declare crates.io-compatible manifests
    When I run the package readiness check
    Then the publishable workspace crates package successfully
