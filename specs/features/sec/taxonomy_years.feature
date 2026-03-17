@REQ-XK-TAXONOMY
@layer.sec
@suite.synthetic
Feature: Taxonomy years

  @alpha-active
  @AC-XK-TAXONOMY-002
  @SCN-XK-TAXONOMY-002
  @speed.fast
  Scenario: Reject mixed-year taxonomy combinations
    Given the profile pack "sec/efm-77/opco"
    And the fixture directory "synthetic/taxonomy/mixed-year-01"
    When I validate the filing
    Then the validation report contains rule "SEC.TAXONOMY.SAME_YEAR"
