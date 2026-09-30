FUNCTION NanoSecToTime : TIME
    VAR_INPUT
        In: LINT;
    END_VAR
END_FUNCTION

FUNCTION GetTime : DATE_AND_TIME
END_FUNCTION

FUNCTION SizeOfAry : UINT
    VAR_IN_OUT
        In: LogEntry;
    END_VAR
END_FUNCTION

FUNCTION EnumToNum : DINT
    VAR_INPUT
        In: LogLevelEnum;
    END_VAR
END_FUNCTION

FUNCTION StackPush : BOOL
    VAR_INPUT
        In: LogEntry;
        Size: UINT;
    END_VAR

    VAR_IN_OUT
        InOut: LogEntry;
        Num: UINT;
    END_VAR
END_FUNCTION
