Feature: Clamp And Saw
    Ensure that clamps always extend before the saw does.

Scenario: 1: Clamps Extend
    Given a machine is running
    And all the Actuators are retracted
    When the Operator holds the THNTD buttons
    Then the Clamps will extend

Scenario: 2: Saw Extends
	Given the Clamps extended
    When the Operator holds the THNTD buttons
    Then the Saw will extend

Scenario: 3: Saw Retracts
    Given the Saw extended
    When 4 seconds have elapsed
    Then the Saw will retract
    And the Clamps will retract
