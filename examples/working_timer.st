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

VAR_GLOBAL
    latched: BOOL := FALSE;
    state: DINT := 0;
END_VAR

FUNCTION_BLOCK main
    VAR
        timer: TON;
    END_VAR

    timer(IN:=TRUE, PT:=T#3s);

    IF timer.Q THEN
        timer(IN:=FALSE, PT:=T#3s);
        state := 10;
        latched := TRUE;

    ELSE
        state := 5;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION GetState : DINT
    GetState := state;
END_FUNCTION
