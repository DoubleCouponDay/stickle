TYPE State : (
    AllStopped := 0,
    ClampsExtending := 1,
    ClampsExtended := 2,
    SawExtending := 3,
    SawExtended := 4,
    SawRetracting := 5,
    SawRetracted := 6,
    ClampsRetracting := 7,
    ClampsRetracted := 8
);
END_TYPE

VAR_GLOBAL
    THNTD: BOOL := FALSE;
END_VAR

{external}
FUNCTION puts : DINT
    VAR_INPUT {ref}
        text : STRING;
    END_VAR
END_FUNCTION

{external}
FUNCTION UINT_TO_STRING : STRING
VAR_INPUT
    input : UINT;
END_VAR
END_FUNCTION

FUNCTION_BLOCK StateMachine
    VAR
        currentState: State := State.AllStopped;
    END_VAR

    VAR_OUTPUT
        output: State;
    END_VAR

    CASE currentState OF
        AllStopped:
            currentState := ClampsExtending;

        ClampsExtending:
            currentState := ClampsExtended;

        ClampsExtended:
            currentState := SawExtending;

        SawExtending:
            currentState := SawExtended;

        SawExtended:
            currentState := SawRetracting;

        SawRetracting:
            currentState := SawRetracted;

        SawRetracted:
            currentState := ClampsRetracting;

        ClampsRetracting:
            currentState := ClampsRetracted;

        ClampsRetracted:
            currentState := AllStopped;
    END_CASE;
    
    output := currentState;
END_FUNCTION_BLOCK

FUNCTION main : UINT
    VAR
        sm : StateMachine;        
        output : UINT;
    END_VAR

    output := sm();
    puts(UINT_TO_STRING(output));
    main := output;
END_FUNCTION
