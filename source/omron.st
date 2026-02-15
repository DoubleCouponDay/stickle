PROGRAM TestAllBuiltins
    VAR_EXTERNAL
        _PLC_OptSerialErrSta: ARRAY[1..2] OF _sOPTSERIALERR_STA;
        _PowerOnCount: UDINT;
        _PowerOnHour: UDINT;
        _Card1PowerFail: BOOL;
        _Card1PrgTransferCmd: _SPRGTRANSFER_CMD;
        _DeviceOutHoldCfg: WORD;
        _EC_StatisticsLogCycleSec: UINT;
        _EC_StatisticsLogEnable: BOOL;
        _EIP_TDLinkStartCmd: BOOL;
        _EIP_TDLinkStopCmd: BOOL;
        _EIP1_TDLinkStartCmd: BOOL;
        _EIP1_TDLinkStopCmd: BOOL;        
    END_VAR

    VAR_EXTERNAL CONSTANT
        _AlarmFlag: WORD;        
        _BackupBusy: BOOL;
        _Card1Access: BOOL;
        _Card1Deteriorated: BOOL;
        _Card1Err: BOOL;
        _Card1PrgTransferSta: _sPRGTRANSFER_STA;
        _Card1Protect: BOOL;
        _Card1Ready: BOOL;
        _CJB_ErrSta: WORD;
        _CJB_MstrErrSta: WORD;
        _CJB_UnitErrSta: ARRAY[0..3, 0..9] OF WORD;
        _CurrentTime: DATE_AND_TIME;
        _DeviceOutHoldStatus: BOOL;
        _EC_CommErrTbl: ARRAY[1..192] OF BOOL;
        _EC_CycleExceeded: BOOL;
        _EC_DisableSlavTbl: ARRAY[1..192] OF BOOL;
        _EC_DisconnSlavTbl: ARRAY[1..192] OF BOOL;
        _EC_EntrySlavTbl: ARRAY[1..192] OF BOOL;
        _EC_ErrSta: WORD;
        _EC_InData1Invalid: BOOL;
        _EC_InData2Invalid: BOOL;
        _EC_InDataInvalid: BOOL;
        _EC_IndataInvalidErr: BOOL;
        _EC_LanHwErr: BOOL;
        _EC_LinkOffErr: BOOL;
        _EC_LinkStatus: BOOL;
        _EC_MacAdrErr: BOOL;
        _EC_MBXSlavTbl: ARRAY[1..192] OF BOOL;
        _EC_MsgErr: BOOL;
        _EC_MstrErr: WORD;
        _EC_NetCfgCmpErr: BOOL;
        _EC_NetCfgErr: BOOL;
        _EC_NetTopologyErr: BOOL;
        _EC_PDActive: BOOL;
        _EC_PDCommErr: BOOL;
        _EC_PDSendErr: BOOL;
        _EC_PDSlavTbl: ARRAY[1..192] OF BOOL;
        _EC_PDTimeoutErr: BOOL;
        _EC_PktMonStop: BOOL;
        _EC_PktSaving: BOOL;
        _EC_PortErr: WORD;
        _EC_RegSlavTbl: ARRAY[1..192] OF BOOL;
        _EC_SlavAdrDupErr: BOOL;
        _EC_SlavAppErr: BOOL;
        _EC_SlavEmergErr: BOOL;
        _EC_SlavErr: WORD;
        _EC_SlavErrTbl: ARRAY[1..192] OF BOOL;
        _EC_SlavInitErr: BOOL;
        _EC_StatisticsLogBusy: BOOL;
        _EC_StatisticsLogErr: BOOL;
        _EIP_BootpErr: BOOL;
        _EIP_CipErr: WORD;
        _EIP_DNSCfgErr: BOOL;
        _EIP_DNSSrvErr: BOOL;
        _EIP_ErrSta: WORD;
        _EIP_EstbTargetSta: ARRAY[0..255] OF BOOL;
        _EIP_EtnCfgErr: BOOL;
        _EIP_EtnOnlineSta: BOOL;
        _EIP_IdentityErr: BOOL;
        _EIP_IPAdrCfgErr: BOOL;
        _EIP_IPAdrDupErr: BOOL;
        _EIP_IPRTblErr: BOOL;
        _EIP_LanHwErr: BOOL;
        _EIP_MacAdrErr: BOOL;
        _EIP_MultiSwONErr: BOOL;
        _EIP_NTPResult: _sNTP_RESULT;
        _EIP_NTPSrvErr: BOOL;
        _EIP_PortErr: WORD;
        _EIP_RegTargetSta: ARRAY[0..255] OF BOOL;
        _EIP_TagAdrErr: BOOL;
        _EIP_TargetNodeErr: ARRAY[0..255] OF BOOL;
        _EIP_TargetPLCErr: ARRAY[0..255] OF BOOL;
        _EIP_TargetPLCModeSta: ARRAY[0..255] OF BOOL;
        _EIP_TcpAppCfgErr: BOOL;
        _EIP_TcpAppErr: WORD;
        _EIP_TDLinkAllRunSta: BOOL;
        _EIP_TDLinkCfgErr: BOOL;
        _EIP_TDLinkErr: BOOL;
        _EIP_TDLinkOpnErr: BOOL;
        _EIP_TDLinkRunSta: BOOL;
        _EIP1_BootpErr: BOOL;
        _EIP1_CipErr: WORD;
        _EIP1_EstbTargetSta: ARRAY[0..255] OF BOOL;
        _EIP1_EtnCfgErr: BOOL;
        _EIP1_EtnOnlineSta: BOOL;
        _EIP1_IdentityErr: BOOL;
        _EIP1_IPAdrCfgErr: BOOL;
        _EIP1_IPAdrDupErr: BOOL;
        _EIP1_LanHwErr: BOOL;
        _EIP1_MacAdrErr: BOOL;
        _EIP1_MultiSwONErr: BOOL;
        _EIP1_PortErr: WORD;
        _EIP1_RegTargetSta: ARRAY[0..255] OF BOOL;
        _EIP1_TagAdrErr: BOOL;
        _EIP1_TargetNodeErr: ARRAY[0..255] OF BOOL;
        _EIP1_TargetPLCErr: ARRAY[0..255] OF BOOL;
        _EIP1_TargetPLCModeSta: ARRAY[0..255] OF BOOL;
        _EIP1_TDLinkAllRunSta: BOOL;
        _EIP1_TDLinkCfgErr: BOOL;
        _EIP1_TDLinkErr: BOOL;
        _EIP1_TDLinkRunSta: BOOL;
        _ErrSta: WORD;
        _HardwareRevision: STRING[2];
        _MC_AX: ARRAY[0..15] OF _sAXIS_REF;
        _MC_AX_ErrSta: ARRAY[0..15] OF WORD;
        _MC_COM: _sCOMMON_REF;
        _MC_ComErrSta: WORD;
        _MC_ErrSta: WORD;
        _MC_GRP: ARRAY[0..7] OF _sGROUP_REF;
        _MC_GRP_ErrSta: ARRAY[0..7] OF WORD;
        _NXB_ErrSta: WORD;
        _NXB_MaxUnitNo: UINT;
        _NXB_MstrErrSta: WORD;
        _NXB_UnitErrFlagTbl: ARRAY[0..8] OF BOOL;
        _NXB_UnitErrStaTbl: ARRAY[1..8] OF WORD;
        _NXB_UnitIOActiveTbl: ARRAY[0..8] OF BOOL;
        _NXB_UnitMsgActiveTbl: ARRAY[0..8] OF BOOL;
        _NXB_UnitRegTbl: ARRAY[0..8] OF BOOL;
        _PLC_ErrSta: WORD;
        _PLC_OptBoardSta: ARRAY[1..2] OF _sOPTBOARD_STA;
        _RetainFail: BOOL;
        _SelfTest_HighTemperature: BOOL;
        _SelfTest_LowBattery: BOOL;
        _SelfTest_LowFanRevolution: BOOL;
        _SystemSWVersion: ARRAY[0..2] OF USINT;
        _Unitversion: ARRAY[0..2] OF USINT;
    END_VAR    

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

    //builtin types
    _Card1PrgTransferCmd.DirName := '';
    _Card1PrgTransferCmd.Exec := TRUE;
    _Card1PrgTransferCmd.Password := '';
    _Card1PrgTransferCmd.TargetIPAdr := TRUE;
    _Card1PrgTransferCmd.TargetMemory := TRUE;
    _Card1PrgTransferCmd.TargetUserProgram := TRUE;
    _Card1PrgTransferCmd.TargetVariable := TRUE;

    _PLC_OptSerialErrSta[0].Error := TRUE;
    _PLC_OptSerialErrSta[0].FramingErr := TRUE;
    _PLC_OptSerialErrSta[0].OverrunErr := TRUE;
    _PLC_OptSerialErrSta[0].ParityErr := TRUE;
    _PLC_OptSerialErrSta[0].Reserve1 := TRUE;
    _PLC_OptSerialErrSta[0].Reserve2 := TRUE;

    //builtin functions
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
TYPE _SPRGTRANSFER_CMD
    : STRUCT
        DirName: STRING[64];
        Exec: BOOL;
        Password: STRING[33];
        TargetIPAdr: BOOL;
        TargetMemory: BOOL;
        TargetUserProgram: BOOL;
        TargetVariable: BOOL;
    END_STRUCT
END_TYPE

{external}
TYPE _sPRGTRANSFER_STA
    : STRUCT
        Active: BOOL;
        Done: BOOL;
        Err: BOOL;
    END_STRUCT
END_TYPE

{external}
TYPE _sNTP_RESULT
    : STRUCT
        ExecNormal: BOOL;
        ExecTime: BOOL;
    END_STRUCT
END_TYPE

{external}
TYPE _sAXIS_REF
    : STRUCT
        Act: _sAXIS_REF_ACT_DATA;
        Cfg: _sAXIS_REF_CFG;
        Cmd: _sAXIS_REF_CMD_DATA;
        Details: _sAXIS_REF_DET;
        Dir: _sAXIS_REF_DIR;
        DrvStatus: _sAxis_REF_STA_DRV;
        MFaultLvl: _sMC_REF_EVENT;
        Obsr: _sMC_REF_EVENT;
        Reserved: ARRAY[0..31] OF BYTE;
        Scale: _sAXIS_REF_SCALE;
        Status: _sAXIS_REF_STA;
    END_STRUCT
END_TYPE

{external}
TYPE _sAXIS_REF_ACT_DATA
    : STRUCT
        Pos: LREAL;
        TimeStamp: ULINT;
        Trq: LREAL;
        Vel: LREAL;
    END_STRUCT
END_TYPE

{external}
TYPE _sAXIS_REF_CFG
    : STRUCT
        AxEnable: _eMC_AXIS_USE;
        AxNo: UINT;
        AxType: _eMC_AXIS_TYPE;
        ExecID: UINT;
        NodeAddress: UINT;
        Reserved: ARRAY[0..7] OF BYTE;
    END_STRUCT
END_TYPE

{external}
TYPE _eMC_AXIS_USE : UINT
    ( _mcNoneAxis, _mcUnusedAxis, _mcUsedAxis );
END_TYPE

{external}
TYPE _eMC_AXIS_TYPE 


{external}
TYPE _sAXIS_REF_CMD_DATA
    : STRUCT
    END_STRUCT
END_TYPE

{external}
TYPE _sAXIS_REF_DET
    : STRUCT
    END_STRUCT
END_TYPE

{external}
TYPE _sAXIS_REF_DIR
    : STRUCT
    END_STRUCT
END_TYPE

{external}
TYPE _sAXIS_REF_STA_DRV
    : STRUCT
    END_STRUCT
END_TYPE

{external}
TYPE _sMC_REF_EVENT
    : STRUCT
    END_STRUCT
END_TYPE

{external}
TYPE _sAXIS_REF_SCALE
    : STRUCT
    END_STRUCT
END_TYPE

{external}
TYPE _sAXIS_REF_STA
    : STRUCT
    END_STRUCT
END_TYPE


{external}
TYPE _sCOMMON_REF

END_TYPE

{external}
TYPE _sGROUP_REF

END_TYPE

{external}
TYPE _sOPTBOARD_STA

END_TYPE

{external}
TYPE _sOPTSERIALERR_STA

END_TYPE

{external}
TYPE HighestLevelStatus : UINT 
    ( NoError, PartialFaultLevel, MinorFaultLevel );
END_TYPE

{external}
TYPE _ePLC_UNIT_TYPE : UINT
    ( _CPU_UNIT );
END_TYPE

{external}
TYPE _sNXUNIT_ID 
    : STRUCT
        NodeAdr: UINT;
        IPAdr: ARRAY[0.55] OF BYTE;
        UnitNo: UDINT;
        Path: ARRAY[0..64] OF BYTE;
        PathLength: USINT;
    END_STRUCT
END_TYPE




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
