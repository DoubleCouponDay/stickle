using Xunit.Gherkin.Quick;
using System;

namespace stickle;

[FeatureFile("../../../features/clampandsaw.feature")]
public class ClampAndSaw : Feature
{
    //Scenario: 1: Clamps Extend

    [Given("a machine is running")]
    public void GivenAMachineIsRunning()
    {
        while(true)
        {
            var state = ClampAndSawFFI.ExecuteStateMachine();
            Console.WriteLine(state);
            Thread.Sleep(1000);
        }
    }

    [And("all the Actuators are retracted")]
    public void AndAllTheActuatorsAreRetracted()
    {
        
    }

    [When("the Operator holds the THNTD buttons")]
    public void WhenTheOperatorHoldsTheTHNTDButtons()
    {

    }

    [Then("the Clamps will extend")]
    public void ThenTheClampsWillExtend()
    {

    }


    //Scenario: 2: Saw Extends

    [Given("the Clamps extended")]
    public void GivenTheClampsExtended()
    {

    }

    [Then("the Saw will extend")]
    public void ThenTheSawWillExtend()
    {

    }

    [Given("the Saw extended")]
    public void GivenTheSawExtended()
    {

    }

    [When("4 seconds have elapsed")]
    public void When4SecondsHaveElapsed()
    {

    }

    [Then("the Saw will retract")]
    public void ThenTheSawWillRetract()
    {

    }

    [And("the Clamps will retract")]
    public void AndTheClampsWillRetract()
    {

    }
}
