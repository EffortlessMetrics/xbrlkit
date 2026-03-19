@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Cockpit pack

  @AC-XK-WORKFLOW-003
  @SCN-XK-WORKFLOW-003
  @alpha-active
  @speed.fast
  Scenario: Wrap a validation report into sensor.report.v1
    Given the profile pack "sec/efm-77/opco"
    Given the fixture "synthetic/workflow/cockpit-test"
    Given a validation report receipt
    When I package the receipt for cockpit
    Then the sensor report is emitted
