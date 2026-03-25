@REQ-XK-CONTEXT
@layer.foundation
@suite.synthetic
Feature: Context completeness validation

  All XBRL facts must reference valid, defined contexts. A fact with
  a context_ref that does not exist in the report is a data integrity
  error that can cause downstream processing failures.

  @alpha-candidate
  @AC-XK-CONTEXT-001
  @SCN-XK-CONTEXT-001
  @speed.fast
  Scenario: Fact references missing context
    Given the fixture "synthetic/contexts/missing-context-01"
    And an XBRL report with context "ctx-valid"
    And a fact referencing concept "us-gaap:Revenue" with context "ctx-missing"
    When context completeness validation runs
    Then a context-missing error is reported for context "ctx-missing"
    And the finding rule ID is "SEC-CONTEXT-001"

  @alpha-candidate
  @AC-XK-CONTEXT-002
  @SCN-XK-CONTEXT-002
  @speed.fast
  Scenario: All facts reference valid contexts
    Given the fixture "synthetic/contexts/valid-contexts-01"
    And an XBRL report with contexts "ctx-2023" and "ctx-2024"
    And facts referencing concepts "us-gaap:Revenue" and "us-gaap:Assets" with contexts "ctx-2023" and "ctx-2024"
    When context completeness validation runs
    Then no context completeness findings are reported

  @alpha-candidate
  @AC-XK-CONTEXT-003
  @SCN-XK-CONTEXT-003
  @speed.fast
  Scenario: Context ID matching is case-insensitive
    Given the fixture "synthetic/contexts/case-insensitive-01"
    And an XBRL report with context "CTX-2024"
    And a fact referencing concept "us-gaap:Revenue" with context "ctx-2024"
    When context completeness validation runs
    Then no context completeness findings are reported

  @alpha-candidate
  @AC-XK-CONTEXT-004
  @SCN-XK-CONTEXT-004
  @speed.fast
  Scenario: Multiple facts with missing contexts
    Given the fixture "synthetic/contexts/multiple-missing-01"
    And an XBRL report with context "ctx-valid"
    And facts referencing concepts "us-gaap:Revenue" and "us-gaap:Assets" with contexts "ctx-missing-1" and "ctx-missing-2"
    When context completeness validation runs
    Then 2 context-missing errors are reported
