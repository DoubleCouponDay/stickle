//with the Rusty Compiler, only functions can be external.

FUNCTION TraceSamp : BOOL
    VAR_INPUT
        TraceNo: USINT;
        Point: USINT;
    END_VAR
END_FUNCTION

FUNCTION TraceTrig : BOOL
    VAR_INPUT
        TraceNo: USINT;
    END_VAR
END_FUNCTION

FUNCTION GetTraceStatus : BOOL
    VAR_INPUT
        TraceNo: USINT;
    END_VAR

    VAR_OUTPUT
        IsStart: BOOL;
        IsComplete: BOOL;
        ParamErr: BOOL;
        IsTrigger: BOOL;
    END_VAR
END_FUNCTION

FUNCTION SetAlarm : BOOL
    VAR_INPUT
        Code: UINT;
        Info1: UINT;
        Info2: UINT;
    END_VAR
END_FUNCTION

FUNCTION ResetAlarm : BOOL
    VAR_INPUT
        Code: UINT;
    END_VAR
END_FUNCTION

FUNCTION GetAlarm : BOOL
    VAR_OUTPUT
        Level: UINT;
        Code: STRING;
    END_VAR
END_FUNCTION

FUNCTION GetPLCError : BOOL
    VAR_INPUT
        Level: UINT;
        Code: DWORD;
    END_VAR
END_FUNCTION

FUNCTION GetEIPError : BOOL
    VAR_INPUT
        Level: UINT;
        Code: DWORD;
    END_VAR
END_FUNCTION

FUNCTION GetMCError : BOOL
    VAR_INPUT
        Level: UINT;
        Code: DWORD;
    END_VAR
END_FUNCTION

FUNCTION GetECError : BOOL
    VAR_INPUT
        Level: UINT;
        Code: DWORD;
    END_VAR
END_FUNCTION

FUNCTION SetInfo : BOOL
    VAR_INPUT
        Code: UINT;
        Info1: UINT;
        Info2: UINT;
    END_VAR
END_FUNCTION

FUNCTION GetNTPStatus : BOOL
    VAR_OUTPUT
        ExecTime: DATE_AND_TIME;
        ExecNormal: BOOL;
    END_VAR
END_FUNCTION

FUNCTION GetMyTaskStatus : BOOL
    VAR_OUTPUT
        LastExecTime: TIME;
        MaxExecTime: TIME;
        MinExecTime: TIME;
        ExecCount: UDINT;
        Exceeded: BOOL;
        ExceedCount: UDINT;
    END_VAR
END_FUNCTION

FUNCTION GetMyTaskInterval : TIME
END_FUNCTION

FUNCTION Task_IsActive : BOOL
    VAR_INPUT
        TaskName: STRING;
    END_VAR
END_FUNCTION

FUNCTION Lock : BOOL
    VAR_INPUT
        Index: USINT;
    END_VAR
END_FUNCTION

FUNCTION Unlock : BOOL
    VAR_INPUT
        Index: USINT;
    END_VAR
END_FUNCTION

FUNCTION ActEventTask : BOOL
    VAR_INPUT
        TaskName: STRING;
    END_VAR
END_FUNCTION

FUNCTION Get10msClk : BOOL
END_FUNCTION

FUNCTION Get10msCnt : BOOL
END_FUNCTION

FUNCTION GetPrgHashCode : BOOL
    VAR_INPUT
        Code1: DWORD;
        Code2: DWORD;
        Code3: DWORD;
        Code4: DWORD;
    END_VAR
END_FUNCTION
