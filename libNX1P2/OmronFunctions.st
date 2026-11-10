FUNCTION NanoSecToTime : TIME
    VAR_INPUT
        In: LINT;
    END_VAR
END_FUNCTION

FUNCTION GetTime : DATE_AND_TIME
END_FUNCTION

FUNCTION SizeOfAry : UINT
    VAR_IN_OUT
        In: ANY;
    END_VAR
END_FUNCTION

FUNCTION EnumToNum : DINT
    VAR_INPUT
        In: DINT;
    END_VAR
END_FUNCTION

FUNCTION StackPush : BOOL
    VAR_INPUT
        In: ANY;
        Size: UINT;
    END_VAR

    VAR_IN_OUT
        InOut: ANY;
        Num: UINT;
    END_VAR
END_FUNCTION

FUNCTION MilliSecToTime : TIME
    VAR_INPUT
        In: DINT;
    END_VAR

MilliSecToTime := NanoSecToTime(In*1000000);
END_FUNCTION
