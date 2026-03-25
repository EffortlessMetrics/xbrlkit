@REQ-XK-SEC-NEGATIVE
@layer.sec
@suite.synthetic
Feature: SEC negative value validation

  SEC filings must not contain negative values for concepts that are
  semantically non-negative by nature (e.g., shares outstanding, 
  employee counts).

  @alpha-candidate
  @AC-XK-SEC-NEGATIVE-001
  @SCN-XK-SEC-NEGATIVE-001
  @speed.fast
  Scenario: Negative share count detected as error
    Given the profile pack "sec/efm-77/opco"
    And an inline XBRL document with fact "dei:EntityCommonStockSharesOutstanding" valued "-1000"
    When the document is validated
    Then the validation report contains a finding with rule ID containing "NEGATIVE_VALUE"
    And the finding severity is "error"
    And the finding subject is "dei:EntityCommonStockSharesOutstanding"

  @alpha-candidate
  @AC-XK-SEC-NEGATIVE-002
  @SCN-XK-SEC-NEGATIVE-002
  @speed.fast
  Scenario: Valid non-negative share count passes validation
    Given the profile pack "sec/efm-77/opco"
    And an inline XBRL document with fact "dei:EntityCommonStockSharesOutstanding" valued "1000000"
    When the document is validated
    Then the validation report has no findings with severity "error"

  @alpha-candidate
  @AC-XK-SEC-NEGATIVE-003
  @SCN-XK-SEC-NEGATIVE-003
  @speed.fast
  Scenario: Negative employee count detected as error
    Given the profile pack "sec/efm-77/opco"
    And an inline XBRL document with fact "dei:EntityNumberOfEmployees" valued "-50"
    When the document is validated
    Then the validation report contains a finding with rule ID containing "NEGATIVE_VALUE"
    And the finding subject is "dei:EntityNumberOfEmployees"

  @alpha-candidate
  @AC-XK-SEC-NEGATIVE-004
  @SCN-XK-SEC-NEGATIVE-004
  @speed.fast
  Scenario: Accounting notation for negative values detected
    Given the profile pack "sec/efm-77/opco"
    And an inline XBRL document with fact "dei:EntityCommonStockSharesOutstanding" valued "(1000)"
    When the document is validated
    Then the validation report contains a finding with rule ID containing "NEGATIVE_VALUE"

  @alpha-candidate
  @AC-XK-SEC-NEGATIVE-005
  @SCN-XK-SEC-NEGATIVE-005
  @speed.fast
  Scenario: Financial loss values can be negative
    Given the profile pack "sec/efm-77/opco"
    And an inline XBRL document with fact "us-gaap:NetIncomeLoss" valued "-5000000"
    When the document is validated
    Then the validation report has no findings with severity "error"
