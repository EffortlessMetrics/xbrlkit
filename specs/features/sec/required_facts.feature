@REQ-XK-SEC-REQUIRED
@layer.sec
@suite.synthetic
Feature: SEC required facts validation

  @alpha-active
  @AC-XK-SEC-REQUIRED-001
  @SCN-XK-SEC-REQUIRED-001
  @speed.fast
  Scenario: Missing required fact detected and reported
    Given the profile pack "sec/efm-77/opco"
    And the fixture directory "synthetic/sec/required-facts/missing-dei-facts"
    When I validate the filing
    Then the validation report contains rule "SEC.REQUIRED_FACT.DEI_ENTITYREGISTRANTNAME"

  @alpha-active
  @AC-XK-SEC-REQUIRED-002
  @SCN-XK-SEC-REQUIRED-002
  @speed.fast
  Scenario: All required facts present passes validation
    Given the profile pack "sec/efm-77/opco"
    And the fixture directory "synthetic/sec/required-facts/valid-dei-facts"
    When I validate the filing
    Then the validation report has no error findings
