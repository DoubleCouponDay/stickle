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
