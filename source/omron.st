//Built In Types
TYPE HighestLevelStatus : UINT 
    ( NoError, PartialFaultLevel := 2, MinorFaultLevel := 3 );
END_TYPE

//Built In Variables

//Built In Functions
	// TraceSamp, GetTraceStatus, TraceTrig,
	
	// SetAlarm, ResetAlarm, GetAlarm,
	
	// ResetPLCError, GetPLCError, ResetCJBError, GetCJBError, GetEIPError, ResetMCError, GetMCError, ResetNXBError, GetNXBerror, GetNXUnitError, ResetXBUnitError, GetXBError,
	
	// SetInfo,
	
	// ResetUnit,
	
	// GetNTPStatus,
	
	// RestartNXUnit,
	
	// NX_ChangeWriteMode, NX_SaveParam,
	
	// PLC_ReadTotalPowerOnTime, NX_ReadTotalPowerOnTime, XBUnit_ReadTotalPowerOnTime,
	
	// APB_ChangeSamplingSettings,
	
	// EC_GetSlaveSatistics,
	
	// NX_DOutTimeStamp, NX_AryDOutTimeStamp
	
	// GetMyTaskStatus, GetMyTaskInterval, Task_IsActive, Lock, Unlock, ActEventTask, GetXXXXClk, GetXXXXCnt, GetPrgHashCode

// Trace Functions
FUNCTION TraceSamp : BOOL
    VAR_INPUT
        TraceNo: DINT;
        Point: DINT;
    END_VAR
END_FUNCTION

FUNCTION TraceTrig : BOOL
    VAR_INPUT
        TraceNo: DINT;
    END_VAR
END_FUNCTION

FUNCTION GetTraceStatus : BOOL
    VAR_INPUT
        TraceNo: DINT;
    END_VAR

    VAR_OUTPUT
        IsStart: BOOL;
        IsComplete: BOOL;
        ParamErr: BOOL;
        IsTrigger: BOOL;
    END_VAR
END_FUNCTION

// Alarm Functions
FUNCTION SetAlarm : BOOL
    VAR_INPUT
        Code: UINT;
        Info1: ANY;
        Info2: ANY;
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
        Code: UINT;
    END_VAR
END_FUNCTION

// Error Functions
FUNCTION ResetPLCError : BOOL
END_FUNCTION

FUNCTION GetPLCError : BOOL
    VAR_INPUT
        Level: HighestLevelStatus;
        Code: DWORD;
    END_VAR
END_FUNCTION

FUNCTION ResetCJBError : BOOL
END_FUNCTION

FUNCTION GetCJBError : BOOL
END_FUNCTION

FUNCTION GetEIPError : BOOL
END_FUNCTION

FUNCTION ResetMCError : BOOL
END_FUNCTION

FUNCTION GetMCError : BOOL
END_FUNCTION

FUNCTION ResetNXBError : BOOL
END_FUNCTION

FUNCTION GetNXBerror : BOOL
END_FUNCTION

FUNCTION GetNXUnitError : BOOL
END_FUNCTION

FUNCTION ResetXBUnitError : BOOL
END_FUNCTION

FUNCTION GetXBError : BOOL
END_FUNCTION

// Info Function
FUNCTION SetInfo : BOOL
END_FUNCTION

// Unit Functions
FUNCTION ResetUnit : BOOL
END_FUNCTION

// NTP Functions
FUNCTION GetNTPStatus : BOOL
END_FUNCTION

// NX Unit Functions
FUNCTION RestartNXUnit : BOOL
END_FUNCTION

FUNCTION NX_ChangeWriteMode : BOOL
END_FUNCTION

FUNCTION NX_SaveParam : BOOL
END_FUNCTION

// Power On Time Functions
FUNCTION PLC_ReadTotalPowerOnTime : BOOL
END_FUNCTION

FUNCTION NX_ReadTotalPowerOnTime : BOOL
END_FUNCTION

FUNCTION XBUnit_ReadTotalPowerOnTime : BOOL
END_FUNCTION

// APB Functions
FUNCTION APB_ChangeSamplingSettings : BOOL
END_FUNCTION

// EtherCAT Functions
FUNCTION EC_GetSlaveSatistics : BOOL
END_FUNCTION

// Timestamp Functions
FUNCTION NX_DOutTimeStamp : BOOL
END_FUNCTION

FUNCTION NX_AryDOutTimeStamp : BOOL
END_FUNCTION

// Task Functions
FUNCTION GetMyTaskStatus : BOOL
END_FUNCTION

FUNCTION GetMyTaskInterval : BOOL
END_FUNCTION

FUNCTION Task_IsActive : BOOL
END_FUNCTION

FUNCTION Lock : BOOL
END_FUNCTION

FUNCTION Unlock : BOOL
END_FUNCTION

FUNCTION ActEventTask : BOOL
END_FUNCTION

FUNCTION GetXXXXClk : BOOL
END_FUNCTION

FUNCTION GetXXXXCnt : BOOL
END_FUNCTION

FUNCTION GetPrgHashCode : BOOL
END_FUNCTION
