TYPE State : (
    AllStopped,
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
    testString: STRING;
    THNTD: BOOL := FALSE;
    stateMachine : StateMachine;
    threeseconds: TON;
END_VAR

FUNCTION main : DINT
    stateMachine(reset:=FALSE);
    main := stateMachine.output;
END_FUNCTION

FUNCTION SetThntd
    VAR_INPUT
        input: BOOL;
    END_VAR
    THNTD := input;
END_FUNCTION

FUNCTION ResetState
    stateMachine(reset:=TRUE);
END_FUNCTION

FUNCTION_BLOCK StateMachine
    VAR_INPUT 
        reset: BOOL;
    END_VAR

    VAR
        currentState: State := AllStopped;
    END_VAR

    VAR_OUTPUT
        output: State;
    END_VAR

    IF reset THEN
        currentState := AllStopped;
        output := currentState;
        RETURN
    END_IF

    CASE currentState OF
        AllStopped:
            IF THNTD THEN
                currentState := ClampsExtending;
            END_IF

        ClampsExtending:
            currentState := ClampsExtended;
            
        ClampsExtended:
            currentState := SawExtending;

        SawExtending:
            currentState := SawExtended;

        SawExtended:
            threeseconds(IN:=TRUE, PT:=T#3s);

            IF threeseconds.Q THEN
                threeseconds(IN:=FALSE, PT:=T#3s); //reset the timer
                currentState := SawRetracting;
            END_IF

        SawRetracting:
            currentState := SawRetracted;

        SawRetracted:
            currentState := ClampsRetracting;

        ClampsRetracting:
            currentState := ClampsRetracted;

        ClampsRetracted:
            currentState := AllStopped;
            THNTD := FALSE;

    END_CASE;
    
    output := currentState;
END_FUNCTION_BLOCK
