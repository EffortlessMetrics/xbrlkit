@REQ-XK-VALIDATION-FINDINGS
@layer.foundation
@suite.synthetic
Feature: Validation finding construction

  @alpha-candidate
  @AC-XK-VALIDATION-FINDINGS-001
  @SCN-XK-VALIDATION-FINDINGS-001
  @speed.fast
  Scenario: Shared constructors preserve finding field mappings
    Given a fact with concept "us-gaap:Revenue" and member "us-gaap:ActualMember"
    And a dimension-member pair with dimension "us-gaap:ScenarioAxis" and member "us-gaap:ActualMember"
    When I construct findings with the shared constructors
    Then the fact finding serializes member "us-gaap:ActualMember" and subject "us-gaap:Revenue"
    And the dimension finding serializes member "us-gaap:ScenarioAxis" and subject "us-gaap:ActualMember"
