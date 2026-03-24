@layer.workflow
@suite.synthetic
Feature: Cockpit pack

  @SCN-XK-WORKFLOW-003
  @speed.fast
  @alpha-active
  Scenario: Wrap a validation report into sensor.report.v1
    Given a validation report receipt
    When I package the receipt for cockpit
    Then the sensor report is emitted
