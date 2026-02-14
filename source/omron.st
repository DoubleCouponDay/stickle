PROGRAM TestAllBuiltins
    VAR
        time1: TIME;
        usint1: USINT;
        type1: _ePLC_UNIT_TYPE;
        info1: UINT;
        dateandtime1: DATE_AND_TIME;
        bool1: BOOL;
        uint1: UINT;
        dword1: DWORD;
        sid1: _sNXUNIT_ID;
        udint1: UDINT;
    END_VAR

    TraceSamp(usint1, usint1);
    TraceTrig(usint1);
    GetTraceStatus(usint1, bool1, bool1, bool1, bool1);
    type1 := _CPU_UNIT;
    SetAlarm(5, info1, info1);
    ResetAlarm(5);
    GetAlarm(uint1, uint1);
    GetPLCError(uint1, dword1);
    GetEIPError(uint1, dword1);
    GetMCError(uint1, dword1);
    GetECError(uint1, dword1);
    GetNXBError(sid1, uint1);
    SetInfo(5, info1, info1);
    GetNTPStatus(dateandtime1, bool1);
    time1 := PLC_ReadTotalPowerOnTime(type1);
    GetMyTaskStatus(time1, time1, time1, udint1, bool1, udint1);
    GetMyTaskInterval();
    Task_IsActive();
    Lock();
    Unlock();
    ActEventTask();
    Get10msClk();
    Get10msCnt();
    GetPrgHashCode(dword1, dword1, dword1, dword1);
END_PROGRAM

//Built In Types

{external}
TYPE HighestLevelStatus : UINT 
    ( NoError, PartialFaultLevel, MinorFaultLevel );
END_TYPE

{external}
TYPE _ePLC_UNIT_TYPE : UINT
    ( _CPU_UNIT );
END_TYPE

{external}
TYPE _sNXUNIT_ID : 
    STRUCT
        NodeAdr: UINT;
        IPAdr: BYTE[5];
        UnitNo: UDINT;
        Path: BYTE[64];
        PathLength: USINT;
    END_STRUCT
END_TYPE

//Built In Variables



//Built In Functions

{external}
FUNCTION TraceSamp : BOOL
    VAR_INPUT
        TraceNo: USINT;
        Point: USINT;
    END_VAR
END_FUNCTION

{external}
FUNCTION TraceTrig : BOOL
    VAR_INPUT
        TraceNo: USINT;
    END_VAR
END_FUNCTION

{external}
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

{external}
FUNCTION SetAlarm<T : ANY_ELEMENTARY> : BOOL
    VAR_INPUT
        Code: UINT;
        Info1: T;
        Info2: T;
    END_VAR
END_FUNCTION

{external}
FUNCTION ResetAlarm : BOOL
    VAR_INPUT
        Code: UINT;
    END_VAR
END_FUNCTION

{external}
FUNCTION GetAlarm : BOOL
    VAR_OUTPUT
        Level: UINT;
        Code: STRING;
    END_VAR
END_FUNCTION

{external}
FUNCTION GetPLCError : BOOL
    VAR_INPUT
        Level: HighestLevelStatus;
        Code: DWORD;
    END_VAR
END_FUNCTION

{external}
FUNCTION GetEIPError : BOOL
    VAR_INPUT
        Level: HighestLevelStatus;
        Code: DWORD;
    END_VAR
END_FUNCTION

{external}
FUNCTION GetMCError : BOOL
    VAR_INPUT
        Level: HighestLevelStatus;
        Code: DWORD;
    END_VAR
END_FUNCTION

{external}
FUNCTION GetECError : BOOL
    VAR_INPUT
        Level: HighestLevelStatus;
        Code: DWORD;
    END_VAR
END_FUNCTION

{external}
FUNCTION GetNXBerror : BOOL
    VAR_INPUT
        UnitProxy: _sNXUNIT_ID;
        Level: HighestLevelStatus;
    END_VAR
END_FUNCTION

{external}
FUNCTION SetInfo<T : ANY_ELEMENTARY> : BOOL
    VAR_INPUT
        Code: UINT;
        Info1: T;
        Info2: T;
    END_VAR
END_FUNCTION

{external}
FUNCTION GetNTPStatus : BOOL
    VAR_OUTPUT
        ExecTime: DATE_AND_TIME;
        ExecNormal: BOOL;
    END_VAR
END_FUNCTION

{external}
FUNCTION PLC_ReadTotalPowerOnTime : TIME
    VAR_INPUT
        UnitType: _ePLC_UNIT_TYPE;
    END_VAR
END_FUNCTION

{external}
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

{external}
FUNCTION GetMyTaskInterval : TIME
END_FUNCTION

{external}
FUNCTION Task_IsActive : BOOL
    VAR_INPUT
        TaskName: STRING;
    END_VAR
END_FUNCTION

{external}
FUNCTION Lock : BOOL
    VAR_INPUT
        Index: USINT;
    END_VAR
END_FUNCTION

{external}
FUNCTION Unlock : BOOL
    VAR_INPUT
        Index: USINT;
    END_VAR
END_FUNCTION

{external}
FUNCTION ActEventTask : BOOL
    VAR_INPUT
        TaskName: STRING;
    END_VAR
END_FUNCTION

{external}
FUNCTION Get10msClk : BOOL
END_FUNCTION

{external}
FUNCTION Get10msCnt : BOOL
END_FUNCTION

{external}
FUNCTION GetPrgHashCode : BOOL
    VAR_INPUT
        Code1: DWORD;
        Code2: DWORD;
        Code3: DWORD;
        Code4: DWORD;
    END_VAR
END_FUNCTION
