Feature: XBRL Dimensional Validation
  As an XBRL processor
  I want to validate dimensional aspects of facts
  So that I can ensure dimension-member pairs are valid according to taxonomy

  Background:
    Given the taxonomy has dimension definitions
    And the taxonomy has domain hierarchies
    And the taxonomy has hypercube definitions

  @alpha-active
  Scenario: Valid dimension-member pair passes validation
    Given a context with dimension "us-gaap:StatementScenarioAxis"
    And the member "us-gaap:ScenarioActualMember"
    When I validate the dimension-member pair
    Then the validation should pass
    And no findings should be reported

  @alpha-active
  Scenario: Invalid dimension-member pair fails validation
    Given a context with dimension "us-gaap:StatementScenarioAxis"
    And an invalid member "us-gaap:NonExistentMember"
    When I validate the dimension-member pair
    Then the validation should fail
    And an "XBRL.DIMENSION.INVALID_MEMBER" finding should be reported

  @alpha-active
  Scenario: Missing required dimension is detected
    Given a fact for concept "us-gaap:Revenue" requiring dimension "us-gaap:StatementScenarioAxis"
    And a context without that dimension
    When I validate the fact dimensions
    Then the validation should fail
    And an "XBRL.DIMENSION.MISSING_REQUIRED" finding should be reported

  @alpha-active
  Scenario: Unknown dimension is rejected
    Given a context with unknown dimension "custom:UnknownAxis"
    When I validate the dimension-member pair
    Then the validation should fail
    And an "XBRL.DIMENSION.UNKNOWN" finding should be reported
