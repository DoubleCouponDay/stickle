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
    currentState: State;
    THNTD: BOOL := FALSE;
    reset: BOOL := FALSE;
    testString: STRING[256];
END_VAR

PROGRAM clampandsaw
    VAR
        threeseconds: TON;
    END_VAR

    IF reset THEN
        reset := FALSE;
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
                threeseconds(IN:=FALSE, PT:=T#3s); //reset the timer
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
            THNTD := FALSE;

    END_CASE;
END_PROGRAM

FUNCTION main : DINT
    clampandsaw();
    main := currentState;
END_FUNCTION

PROGRAM SetThntd
    VAR_INPUT
        input: BOOL;
    END_VAR

    THNTD := input;
END_PROGRAM

FUNCTION ResetState
    reset:=TRUE;
END_FUNCTION
