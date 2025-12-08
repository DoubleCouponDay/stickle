using Xunit.Gherkin.Quick;

namespace stickle;

[FeatureFile("../../../features/clampandsaw.feature")]
public class ClampAndSaw : Feature
{
    //Scenario: 1: Clamps Extend

    [Given("a machine is running")]
    public void GivenAMachineIsRunning()
    {
        FFI.helloworld_main();
    }

    [And("the Saw is retracted")]
    public void AndTheSawIsRetracted()
    {

    }

    [And("the Clamps are retracted")]
    public void AndTheClampsAreRetracted()
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
