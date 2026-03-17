@REQ-XK-DUPLICATES
@layer.standard
@suite.synthetic
Feature: Duplicate facts

  @alpha-active
  @AC-XK-DUPLICATES-001
  @SCN-XK-DUPLICATES-001
  @speed.fast
  Scenario: Consolidate consistent duplicates
    Given the fixture directory "synthetic/facts/consistent-duplicates-01"
    When I validate duplicate facts
    Then the validation report contains rule "XBRL.DUPLICATE_FACT.CONSISTENT"
    And the validation report does not contain rule "XBRL.DUPLICATE_FACT.INCONSISTENT"
    And the validation report has no error findings
