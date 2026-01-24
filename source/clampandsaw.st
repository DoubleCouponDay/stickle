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

{external}
FUNCTION_BLOCK TON
VAR_INPUT
    IN: BOOL;
    PT: TIME;
END_VAR
VAR_OUTPUT
    Q: BOOL;
    ET: TIME;
END_VAR
VAR
  __signal__ : BOOL; (* Value representing the internal signal *)
  __is_running__: BOOL; (* Internal flag to track timer on/off state *)
  __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
END_VAR
END_FUNCTION_BLOCK

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

FUNCTION PRG_ClampAndSaw : DINT
    VAR
        stateMachine : FB_ClampAndSaw;
    END_VAR

    stateMachine();
END_FUNCTION

FUNCTION_BLOCK FB_ClampAndSaw
    VAR
        threeseconds: TON;
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
