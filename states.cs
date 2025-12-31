
namespace stickle;

public enum States : int
{
    NoneGiven = -1,
    AllStopped,
    ClampsExtending, ClampsExtended,
    SawExtending, SawExtended,
    SawRetracting, SawRetracted,
    ClampsRetracting, ClampsRetracted,
}

public static class StateExtensions
{
    public static States ParseState(this int inputState)
    {
        States output = States.NoneGiven;

        switch(inputState)
        {
            case (int)States.AllStopped:
                output = States.AllStopped;
                break;

            case (int)States.ClampsExtending:
                output = States.ClampsExtending;
                break;

            case (int)States.ClampsExtended:
                output = States.ClampsExtended;            
                break;
                
            case (int)States.SawExtending:
                output = States.SawExtending;
                break;

            case (int)States.SawExtended:
                output = States.SawExtended;
                break;

            case (int)States.SawRetracting:
                output = States.SawRetracting;
                break;

            case (int)States.SawRetracted:
                output = States.SawRetracted;
                break;

            case (int)States.ClampsRetracting:
                output = States.ClampsRetracting;
                break;

            case (int)States.ClampsRetracted:
                output = States.ClampsRetracted;
                break;
        }        
        return output;
    }
}
