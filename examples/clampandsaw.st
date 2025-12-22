{external}
FUNCTION puts
    VAR_INPUT
        in: STRING;
    END_VAR
END_FUNCTION

{external}
FUNCTION SQRT<T : ANY_REAL> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

{external}
FUNCTION REAL_TO_DINT : DINT
VAR_INPUT
    in : REAL;
END_VAR
END_FUNCTION

{external}
FUNCTION DINT_TO_STRING : STRING
    VAR_INPUT
        input: DINT;
    END_VAR
END_FUNCTION

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
    VAR
        test1: REAL;
    END_VAR

    sm();
    main := sm.output;
    
    test1 := SQRT(555);
    main := REAL_TO_DINT(test1);
    puts(DINT_TO_STRING(main));
END_FUNCTION
