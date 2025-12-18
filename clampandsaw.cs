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
        // var cwd = Directory.GetCurrentDirectory();

        // Console.WriteLine($"cwd: {cwd}");
        // var files = Directory.GetFiles(cwd + "/../../../shared/");
        // foreach (var file in files)
        // {
        //     Console.WriteLine($"file: {file}");
        // }

        // while(true) {
        //     int state = ClampAndSawFFI.main();
        //     Console.WriteLine($"state: {state}");
        //     Thread.Sleep(1000);
        // }     

        try
        {
            ClampAndSawFFI.DINT_TO_STRING(42);
            ClampAndSawFFI.main();
        }

        catch(Exception e)
        {
            Console.WriteLine(e);
        }
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
