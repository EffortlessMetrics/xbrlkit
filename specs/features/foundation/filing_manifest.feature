@REQ-XK-MANIFEST
@layer.standard
@suite.synthetic
Feature: Filing manifest

  @alpha-active
  @AC-XK-MANIFEST-001
  @SCN-XK-MANIFEST-001
  @speed.fast
  Scenario: Build a manifest from a minimal filing container
    Given the fixture "synthetic/filing/minimal-container-01"
    When I build the filing manifest
    Then the filing manifest receipt is emitted
