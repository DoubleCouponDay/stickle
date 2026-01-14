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

TYPE TestStruct : 
    STRUCT
        testString: String[512];
    END_STRUCT
END_TYPE

VAR_GLOBAL
    currentState: State := AllStopped;
    THNTD: BOOL := FALSE;
    reset: BOOL := FALSE;
    testString: STRING[256];
END_VAR

PROGRAM PRG_ClampAndSaw
    VAR
        stateMachine : FB_ClampAndSaw;
    END_VAR

    stateMachine(THNTD := THNTD, reset := reset);
END_PROGRAM

FUNCTION_BLOCK FB_ClampAndSaw
    VAR_INPUT
        THNTD : BOOL;
        reset : BOOL;
    END_VAR
    VAR
        threeseconds : TON;
    END_VAR

    IF reset THEN
        reset := false;
        currentState := AllStopped;
        RETURN;
    END_IF;

    CASE currentState OF
        AllStopped:
            IF THNTD THEN
                currentState := ClampsExtending;
            END_IF;

        ClampsExtending:
            currentState := ClampsExtended;

        ClampsExtended:
            currentState := SawExtending;

        SawExtending:
            currentState := SawExtended;

        SawExtended:
            threeseconds(IN := TRUE, PT := T#3s);

            IF threeseconds.Q THEN
                threeseconds(IN := FALSE, PT := T#3s);
                currentState := SawRetracting;
            END_IF;

        SawRetracting:
            currentState := SawRetracted;

        SawRetracted:
            currentState := ClampsRetracting;

        ClampsRetracting:
            currentState := ClampsRetracted;

        ClampsRetracted:
            currentState := AllStopped;
    END_CASE;
END_FUNCTION_BLOCK

FUNCTION GetState : DINT
    GetState := currentState;
END_FUNCTION

FUNCTION SetThntd
    VAR_INPUT
        input: BOOL;
    END_VAR

    THNTD := input;
END_FUNCTION

FUNCTION ResetState
    reset:=TRUE;
END_FUNCTION
