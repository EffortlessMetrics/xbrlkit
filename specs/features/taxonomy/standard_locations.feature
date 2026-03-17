@REQ-XK-TAXONOMY
@layer.sec
@suite.synthetic
Feature: Standard taxonomy locations

  @alpha-active
  @AC-XK-TAXONOMY-001
  @SCN-XK-TAXONOMY-001
  @speed.fast
  Scenario: Resolve a standard taxonomy location via the profile pack
    Given the profile pack "sec/efm-77/opco"
    And the fixture directory "synthetic/taxonomy/standard-location-01"
    When I resolve the DTS
    Then the taxonomy resolution succeeds
    And the taxonomy resolution resolves at least 1 namespaces
