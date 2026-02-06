
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
        testString: STRING;
    END_STRUCT
END_TYPE

VAR_GLOBAL
    currentState: State := AllStopped;
    THNTD: BOOL := FALSE;
    reset: BOOL := FALSE;
    testString: STRING[256];
END_VAR

FUNCTION main : DINT //the main entrypoint must be a function, not a program. Otherwise, timers will cause segfaults
    PRG_ClampAndSaw();
END_FUNCTION

PROGRAM PRG_ClampAndSaw
    VAR
        stateMachine: FB_ClampAndSaw;
    END_VAR

    stateMachine();
END_PROGRAM


FUNCTION_BLOCK FB_ClampAndSaw
    VAR
        threeseconds: TON; //timers can only be referenced by Programs or other Function Blocks
    END_VAR

    VAR_EXTERNAL
        currentState: State;
        THNTD: BOOL;
        reset: BOOL;
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
            threeseconds(IN:=TRUE, PT:=T#3s);

            IF threeseconds.Q THEN
                threeseconds(IN:=FALSE, PT:=T#3s);
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
    VAR_EXTERNAL
        currentState: State;
    END_VAR

    GetState := currentState;
END_FUNCTION

FUNCTION SetThntd
    VAR_INPUT
        input: BOOL;
    END_VAR

    VAR_EXTERNAL
        THNTD: BOOL;
    END_VAR

    THNTD := input;
END_FUNCTION

FUNCTION ResetState
    VAR_EXTERNAL
        reset: BOOL;
    END_VAR

    reset:=TRUE;
END_FUNCTION
