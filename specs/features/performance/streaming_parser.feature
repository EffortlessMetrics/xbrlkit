@alpha-future
Feature: Streaming Parser for Large Files
  As an XBRL validator
  I want to validate large SEC filings without excessive memory usage
  So that I can process 100MB+ files efficiently

  Background:
    Given the xbrl-stream crate is available

  @streaming @memory @SCN-XK-STREAM-001
  Scenario: SCN-XK-STREAM-001 — Stream parse a large XBRL file
    Given an XBRL filing larger than 100MB
    When I validate it using the streaming parser
    Then memory usage should stay under 50MB peak
    And all facts should be processed
    And context references should be validated

  @streaming @fallback @SCN-XK-STREAM-002
  Scenario: SCN-XK-STREAM-002 — Automatic fallback for small files
    Given an XBRL filing smaller than 10MB
    When I check if streaming is needed
    Then the DOM parser should be recommended
    And the streaming parser should be available as option

  @streaming @context @SCN-XK-STREAM-003
  Scenario: SCN-XK-STREAM-003 — Context completeness via streaming
    Given a large XBRL filing with 1000+ facts
    And some facts reference non-existent contexts
    When I run streaming context validation
    Then missing context references should be reported
    And line numbers should indicate error locations

  @streaming @handler @SCN-XK-STREAM-004
  Scenario: SCN-XK-STREAM-004 — Custom fact handler
    Given a streaming parser with a custom handler
    When facts are encountered during parsing
    Then the handler should receive each fact
    And contexts should be collected
    And units should be available for reference
