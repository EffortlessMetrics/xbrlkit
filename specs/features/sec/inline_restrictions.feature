@REQ-XK-SEC-INLINE
@layer.sec
@suite.synthetic
Feature: SEC inline restrictions

  @alpha-active
  @AC-XK-SEC-INLINE-001
  @SCN-XK-SEC-INLINE-001
  @speed.fast
  Scenario: Reject ix:fraction in inline content
    Given the profile pack "sec/efm-77/opco"
    And the fixture directory "synthetic/sec/inline/ix-fraction-01"
    When I validate the filing
    Then the validation report contains rule "SEC.INLINE.NO_IX_FRACTION"
