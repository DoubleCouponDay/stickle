using Xunit.Gherkin.Quick;
using System.Diagnostics;
using System;

namespace stickle;

[FeatureFile("../../../features/clampandsaw.feature")]
public class ClampAndSaw : Feature
{
    States currentState = States.NoneGiven;

    public ClampAndSaw()
    {
        ClampAndSawFFI.ResetState();
        ClampAndSawFFI.SetThntd(false);
    }

    //Scenario: 1: Clamps Extend

    [Given("all the Actuators are retracted")]
    public void GivenAllTheActuatorsAreRetracted()
    {
        Console.WriteLine(nameof(GivenAllTheActuatorsAreRetracted));
        ClampAndSawFFI.PRG_ClampAndSaw();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.AllStopped, currentState);
    }

    [When("the Operator holds the THNTD buttons")]
    public void WhenTheOperatorHoldsTheTHNTDButtons()
    {
        Console.WriteLine(nameof(WhenTheOperatorHoldsTheTHNTDButtons));
        ClampAndSawFFI.SetThntd(true);
    }

    [Then("the Clamps will extend")]
    public void ThenTheClampsWillExtend()
    {
        Console.WriteLine(nameof(ThenTheClampsWillExtend));
        ClampAndSawFFI.PRG_ClampAndSaw();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.ClampsExtending, currentState);
        ClampAndSawFFI.PRG_ClampAndSaw();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.ClampsExtended, currentState);
    }


    //Scenario: 2: Saw Extends

    [Given("the Clamps extended")]
    public void GivenTheClampsExtended()
    {
        Console.WriteLine(nameof(GivenTheClampsExtended));
        GivenAllTheActuatorsAreRetracted();
        WhenTheOperatorHoldsTheTHNTDButtons();
        ThenTheClampsWillExtend();
    }

    [Then("the Saw will extend")]
    public void ThenTheSawWillExtend()
    {
        Console.WriteLine(nameof(ThenTheSawWillExtend));
        ClampAndSawFFI.PRG_ClampAndSaw();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.SawExtending, currentState);
        ClampAndSawFFI.PRG_ClampAndSaw();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.SawExtended, currentState);
    }


    //Scenario: 3: Saw Retracts

    [Given("the Saw extended")]
    public void GivenTheSawExtended()
    {
        Console.WriteLine(nameof(GivenTheSawExtended));
        GivenTheClampsExtended();
        ThenTheSawWillExtend();
    }

    [When("3 seconds have elapsed")]
    public void When3SecondsHaveElapsed()
    {
        Console.WriteLine(nameof(When3SecondsHaveElapsed));
        var counter = new Stopwatch();
        counter.Start();
        Assert.Equal(States.SawExtended, currentState);

        do //stop iterating the moment the state changes or the limit of 4 seconds is reached
        {
            ClampAndSawFFI.PRG_ClampAndSaw();
            currentState = ClampAndSawFFI.GetState().ParseState();
        }

        while(currentState == States.SawExtended && counter.Elapsed < TimeSpan.FromSeconds(4));
        counter.Stop();
    }

    [Then("the Saw will retract")]
    public void ThenTheSawWillRetract()
    {
        Console.WriteLine(nameof(ThenTheSawWillRetract));
        Assert.Equal(States.SawRetracting, currentState);
        ClampAndSawFFI.PRG_ClampAndSaw();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.SawRetracted, currentState);
    }

    [And("the Clamps will retract")]
    public void AndTheClampsWillRetract()
    {
        Console.WriteLine(nameof(AndTheClampsWillRetract));
        ClampAndSawFFI.PRG_ClampAndSaw();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.ClampsRetracting, currentState);

        ClampAndSawFFI.PRG_ClampAndSaw();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.ClampsRetracted, currentState);

        ClampAndSawFFI.PRG_ClampAndSaw();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.AllStopped, currentState);
    }
}
