TYPE State : (
    AllStopped,
    ClampsExtending,
    ClampsExtended,
    SawExtending,
    SawExtended,
    SawRetracting,
    SawRetracted,
    ClampsRetracting,
    ClampsRetracted
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

FUNCTION_BLOCK StateMachine
    VAR
        currentState: State;
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

FUNCTION main : DINT
    VAR
        sm : StateMachine;
    END_VAR

    main := sm();
END_FUNCTION
