@REQ-XK-PERIOD-MODEL
@layer.foundation
@suite.synthetic
Feature: Shared XBRL period model

  Parsed contexts and streaming contexts expose one interoperable period model
  so callers can pass values between both ingestion APIs without conversion.

  @alpha-active
  @AC-XK-PERIOD-001
  @SCN-XK-PERIOD-001
  @speed.fast
  Scenario: Parsed and streaming periods share one model
    Given an instant period from the context API
    When I pass the context period to the streaming API
    Then the parsed and streaming periods should be equal
