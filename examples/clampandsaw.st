
{external}
FUNCTION puts : DINT
VAR_INPUT
    text : STRING;
END_VAR
END_FUNCTION

{external}
FUNCTION printf
VAR_INPUT
    text : STRING;
END_VAR
END_FUNCTION

{external}
FUNCTION DINT_TO_STRING : STRING
VAR_INPUT
    IN : DINT;
END_VAR
END_FUNCTION

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
    currentState: State;
END_VAR

FUNCTION_BLOCK StateMachine
    VAR_OUTPUT
        output: State;
    END_VAR
    puts('StateMachine');

    CASE currentState OF
        AllStopped:
            puts('ClampsExtending');
            currentState := ClampsExtending;

        ClampsExtending:
            puts('ClampsExtended');
            currentState := ClampsExtended;

        ClampsExtended:
            puts('SawExtending');
            currentState := SawExtending;

        SawExtending:
            puts('SawExtending');
            currentState := SawExtended;

        SawExtended:
            puts('ClampsExtended');
            currentState := SawRetracting;

        SawRetracting:
            puts('ClampsExtended');
            currentState := SawRetracted;

        SawRetracted:
            puts('ClampsExtended');        
            currentState := ClampsRetracting;

        ClampsRetracting:
            puts('ClampsExtended');        
            currentState := ClampsRetracted;

        ClampsRetracted:
            puts('ClampsExtended');        
            currentState := AllStopped;
    END_CASE;
    
    output := currentState;
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        sm : StateMachine;
    END_VAR

    sm();
    printf('currentstate: ');
    puts(DINT_TO_STRING(currentState));
    main := currentState;
END_FUNCTION
