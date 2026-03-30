@REQ-XK-TAXONOMY-LOADER
@layer.taxonomy
@suite.synthetic
Feature: Taxonomy Loader
  As an XBRL processor
  I want to load dimension definitions from actual taxonomy files
  So that I can validate facts against real US-GAAP and SEC taxonomies

  Background:
    Given the taxonomy loader is available

  @alpha-active @SCN-XK-TAX-LOAD-001 @AC-XK-TAX-LOAD-001
  @speed.fast
  Scenario: Load dimension definitions from schema
    Given a taxonomy schema with dimension elements
    When I load the taxonomy
    Then the taxonomy should contain dimensions
    And explicit dimensions should have domains

  @alpha-active @SCN-XK-TAX-LOAD-002 @AC-XK-TAX-LOAD-002
  @speed.fast
  Scenario: Load domain hierarchies from definition linkbase
    Given a taxonomy definition linkbase with domain members
    When I load the taxonomy
    Then domains should have members
    And members should maintain parent-child relationships

  @alpha-active @SCN-XK-TAX-LOAD-003 @AC-XK-TAX-LOAD-003
  @speed.fast
  Scenario: Load typed dimension definitions
    Given a taxonomy with typed dimensions
    When I load the taxonomy
    Then typed dimensions should have value types
    And the value types should be valid XSD types

  @alpha-active @SCN-XK-TAX-LOAD-004 @AC-XK-TAX-LOAD-004
  @speed.fast
  Scenario: Load hypercube definitions
    Given a taxonomy with hypercube elements
    When I load the taxonomy
    Then hypercubes should contain their dimensions
    And dimensions should reference their domains

  @alpha-active @SCN-XK-TAX-LOAD-005 @AC-XK-TAX-LOAD-005
  @speed.fast
  Scenario: Cache taxonomy files locally
    Given a taxonomy URL to load
    And a cache directory is configured
    When I load the taxonomy
    Then the taxonomy file should be cached
    And subsequent loads should use the cache

  @alpha-active @SCN-XK-TAX-LOAD-006 @AC-XK-TAX-LOAD-006
  @speed.fast
  Scenario: Handle schema imports recursively
    Given a taxonomy schema that imports another schema
    When I load the taxonomy
    Then imported schemas should be loaded
    And all dimension definitions should be available

  @alpha-active @SCN-XK-TAX-LOAD-007 @AC-XK-TAX-LOAD-007
  @speed.fast
  Scenario: Validate dimension-member against loaded taxonomy
    Given a loaded taxonomy with dimension definitions
    And a context with dimension "us-gaap:StatementScenarioAxis"
    And the member "us-gaap:ScenarioActualMember"
    When I validate the dimension-member pair
    Then the validation should pass

  @alpha-active @SCN-XK-TAX-LOAD-008 @AC-XK-TAX-LOAD-008
  @speed.fast
  Scenario: Reject invalid member against loaded taxonomy
    Given a loaded taxonomy with dimension definitions
    And a context with dimension "us-gaap:StatementScenarioAxis"
    And an invalid member "custom:InvalidMember"
    When I validate the dimension-member pair
    Then the validation should fail
    And an "XBRL.DIMENSION.INVALID_MEMBER" finding should be reported
