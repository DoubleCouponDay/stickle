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
        ClampAndSawFFI.ExecuteStateMachine();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.AllStopped, currentState);
    }

    [When("the Operator holds the THNTD buttons")]
    public void WhenTheOperatorHoldsTheTHNTDButtons()
    {
        ClampAndSawFFI.SetThntd(true);
    }

    [Then("the Clamps will extend")]
    public void ThenTheClampsWillExtend()
    {
        ClampAndSawFFI.ExecuteStateMachine();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.ClampsExtending, currentState);
        ClampAndSawFFI.ExecuteStateMachine();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.ClampsExtended, currentState);
    }


    //Scenario: 2: Saw Extends

    [Given("the Clamps extended")]
    public void GivenTheClampsExtended()
    {
        GivenAllTheActuatorsAreRetracted();
        WhenTheOperatorHoldsTheTHNTDButtons();
        ThenTheClampsWillExtend();        
    }

    [Then("the Saw will extend")]
    public void ThenTheSawWillExtend()
    {
        ClampAndSawFFI.ExecuteStateMachine();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.SawExtending, currentState);
        ClampAndSawFFI.ExecuteStateMachine();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.SawExtended, currentState);
    }


    //Scenario: 3: Saw Retracts

    [Given("the Saw extended")]
    public void GivenTheSawExtended()
    {
        GivenTheClampsExtended();
        ThenTheSawWillExtend();
    }

    [When("3 seconds have elapsed")]
    public void When3SecondsHaveElapsed()
    {
        var counter = new Stopwatch();
        counter.Start();
        Assert.Equal(States.SawExtended, currentState);

        do //stop iterating the moment the state changes or the limit of 4 seconds is reached
        {
            ClampAndSawFFI.ExecuteStateMachine();
            currentState = ClampAndSawFFI.GetState().ParseState();
        }

        while(currentState == States.SawExtended && counter.Elapsed < TimeSpan.FromSeconds(4));
        counter.Stop();
        Assert.Equal(3, counter.Elapsed.Seconds);
    }

    [Then("the Saw will retract")]
    public void ThenTheSawWillRetract()
    {
        Assert.Equal(States.SawRetracting, currentState);
        ClampAndSawFFI.ExecuteStateMachine();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.SawRetracted, currentState);        
    }

    [And("the Clamps will retract")]
    public void AndTheClampsWillRetract()
    {
        ClampAndSawFFI.ExecuteStateMachine();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.ClampsRetracting, currentState);

        ClampAndSawFFI.ExecuteStateMachine();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.ClampsRetracted, currentState);

        ClampAndSawFFI.ExecuteStateMachine();
        currentState = ClampAndSawFFI.GetState().ParseState();
        Assert.Equal(States.AllStopped, currentState);        
    }
}
