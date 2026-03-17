@REQ-XK-IXDS
@layer.standard
@suite.synthetic
Feature: IXDS assembly

  @alpha-active
  @AC-XK-IXDS-001
  @SCN-XK-IXDS-001
  @speed.fast
  Scenario: Assemble a single-file IXDS
    Given the profile pack "sec/efm-77/opco"
    And the fixture directory "synthetic/inline/ixds-single-file-01"
    When I validate the filing
    Then the IXDS assembly receipt contains 1 member
    And the report contains 1 facts
    And the validation report has no error findings
    And the concept set is:
      | dei:DocumentType |

  @alpha-active
  @AC-XK-IXDS-002
  @SCN-XK-IXDS-002
  @speed.fast
  Scenario: Assemble a multi-file IXDS
    Given the profile pack "sec/efm-77/opco"
    And the fixture directory "synthetic/inline/ixds-two-file-01"
    When I validate the filing
    Then the IXDS assembly receipt contains 2 members
    And the report contains 2 facts
    And the validation report has no error findings
    And the concept set is:
      | dei:DocumentType |
      | dei:EntityRegistrantName |
