
TYPE State : (
    AllRetracted,
    ClampsExtending,
    ClampsExtended,
    SawExtending,
    SawExtended,
    SawRetracting,
    SawRetracted,
    ClampsRetracting,
    ClampsRetracted,
);
END_TYPE

VAR_GLOBAL
    THNTD: BOOL := FALSE;
    sm : StateMachine;
END_VAR

FUNCTION_BLOCK StateMachine
    VAR
        currentState: State := AllRetracted;
    END_VAR

    VAR_OUTPUT
        output: State;
    END_VAR

    CASE currentState OF
        AllRetracted:
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
            currentState := AllRetracted;
    END_CASE;
    
    output := currentState;
END_FUNCTION_BLOCK

FUNCTION main : DINT
    sm();
    main := sm.output;
END_FUNCTION
