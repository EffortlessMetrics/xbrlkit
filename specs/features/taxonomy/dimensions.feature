Feature: XBRL Dimensional Validation
  As an XBRL processor
  I want to validate dimensional aspects of facts
  So that I can ensure dimension-member pairs are valid according to taxonomy

  Background:
    Given the taxonomy has dimension definitions
    And the taxonomy has domain hierarchies
    And the taxonomy has hypercube definitions

  @alpha-active @SCN-XK-DIM-001 @AC-XK-DIM-001
  Scenario: Valid dimension-member pair passes validation
    Given a context with dimension "us-gaap:StatementScenarioAxis"
    And the member "us-gaap:ScenarioActualMember"
    When I validate the dimension-member pair
    Then the validation should pass
    And no findings should be reported

  @alpha-active @SCN-XK-DIM-002 @AC-XK-DIM-002
  Scenario: Invalid dimension-member pair fails validation
    Given a context with dimension "us-gaap:StatementScenarioAxis"
    And an invalid member "us-gaap:NonExistentMember"
    When I validate the dimension-member pair
    Then the validation should fail
    And an "XBRL.DIMENSION.INVALID_MEMBER" finding should be reported

  @alpha-active @SCN-XK-DIM-003 @AC-XK-DIM-003
  Scenario: Missing required dimension is detected
    Given a fact for concept "us-gaap:Revenue"
    And the concept requires dimension "us-gaap:StatementScenarioAxis"
    And a context without that dimension
    When I validate the fact dimensions
    Then the validation should fail
    And an "XBRL.DIMENSION.MISSING_REQUIRED" finding should be reported

  @alpha-active @SCN-XK-DIM-004 @AC-XK-DIM-004
  Scenario: Unknown dimension is rejected
    Given a context with unknown dimension "custom:UnknownAxis"
    When I validate the dimension-member pair
    Then the validation should fail
    And an "XBRL.DIMENSION.UNKNOWN" finding should be reported

  @alpha-active @SCN-XK-DIM-005 @AC-XK-DIM-005
  Scenario: Typed member dimension is parsed correctly
    Given a context with typed dimension "dim:CustomerAxis"
    And the typed member value "CUST-12345"
    When I parse the context dimensions
    Then the dimension should be marked as typed
    And the typed value should be "CUST-12345"
    And the member should be "CUST-12345"

  @alpha-active @SCN-XK-DIM-006 @AC-XK-DIM-006
  Scenario: Mixed explicit and typed members in same context
    Given a context with dimension "us-gaap:StatementScenarioAxis"
    And the member "us-gaap:ScenarioActualMember"
    And a typed dimension "dim:ProductAxis"
    And the typed member value "PROD-789"
    When I parse the context dimensions
    Then the explicit dimension should have member "us-gaap:ScenarioActualMember"
    And the typed dimension should have value "PROD-789"
    And both dimensions should be accessible

  @alpha-active @SCN-XK-DIM-007 @AC-XK-DIM-007
  Scenario: Typed member in segment container
    Given a context with typed dimension "dim:EntityIdentifierAxis" in segment
    And the typed member value "ENT-98765"
    When I parse the context dimensions
    Then the typed dimension should be in the entity segment
    And the typed value should be "ENT-98765"

  @alpha-active @SCN-XK-DIM-008 @AC-XK-DIM-008
  Scenario: Empty typed member value is handled
    Given a context with typed dimension "dim:OptionalAxis"
    And the typed member value ""
    When I parse the context dimensions
    Then the dimension should be marked as typed
    And the typed value should be empty
