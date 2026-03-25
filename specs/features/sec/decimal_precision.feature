@REQ-XK-SEC-DECIMAL
@layer.sec
@suite.synthetic
Feature: Decimal Precision Validation (EFM 6.5.37)

  As an XBRL validator
  I want to validate that numeric facts don't truncate non-zero digits
  So that SEC filings maintain numeric data quality and compliance

  Background:
    Given the system has loaded the SEC validation rules

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-001
  @SCN-XK-SEC-DECIMAL-001
  @speed.fast
  Scenario: Valid exact value with INF decimals
    Given a numeric fact with value "1234.56" and decimals "INF"
    When decimal precision validation is performed
    Then no validation errors are reported

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-001
  @SCN-XK-SEC-DECIMAL-002
  @speed.fast
  Scenario: Valid rounded value with appropriate decimals
    Given a numeric fact with value "1234.00" and decimals "0"
    When decimal precision validation is performed
    Then no validation errors are reported

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-001
  @SCN-XK-SEC-DECIMAL-003
  @speed.fast
  Scenario: Invalid truncation of fractional digits
    Given a numeric fact with value "1234.56" and decimals "0"
    When decimal precision validation is performed
    Then validation error "NonzeroDigitsTruncated" is reported

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-001
  @SCN-XK-SEC-DECIMAL-004
  @speed.fast
  Scenario: Invalid truncation of significant digits
    Given a numeric fact with value "1234" and decimals "-3"
    When decimal precision validation is performed
    Then validation error "NonzeroDigitsTruncated" is reported

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-002
  @SCN-XK-SEC-DECIMAL-005
  @speed.fast
  Scenario: Valid high-magnitude rounding with zeros
    Given a numeric fact with value "1000000" and decimals "-5"
    When decimal precision validation is performed
    Then no validation errors are reported

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-002
  @SCN-XK-SEC-DECIMAL-006
  @speed.fast
  Scenario: Valid EPS with two decimal places
    Given a numeric fact with value "1.23" and decimals "2"
    When decimal precision validation is performed
    Then no validation errors are reported

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-001
  @SCN-XK-SEC-DECIMAL-007
  @speed.fast
  Scenario: Invalid EPS with excessive precision
    Given a numeric fact with value "1.234" and decimals "2"
    When decimal precision validation is performed
    Then validation error "NonzeroDigitsTruncated" is reported

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-002
  @SCN-XK-SEC-DECIMAL-008
  @speed.fast
  Scenario: Valid thousands rounding
    Given a numeric fact with value "1234000" and decimals "-3"
    When decimal precision validation is performed
    Then no validation errors are reported

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-002
  @SCN-XK-SEC-DECIMAL-009
  @speed.fast
  Scenario: Valid negative value with INF decimals
    Given a numeric fact with value "-2345.67" and decimals "INF"
    When decimal precision validation is performed
    Then no validation errors are reported

  @alpha-candidate
  @AC-XK-SEC-DECIMAL-001
  @SCN-XK-SEC-DECIMAL-010
  @speed.fast
  Scenario: Invalid negative value truncation
    Given a numeric fact with value "-2345.67" and decimals "0"
    When decimal precision validation is performed
    Then validation error "NonzeroDigitsTruncated" is reported
