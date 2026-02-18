VAR_GLOBAL
    // _PLC_OptSerialErrSta: ARRAY[1..2] OF _sOPTSERIALERR_STA;
    _PowerOnCount: UDINT;
    _PowerOnHour: UDINT;
    _Card1PowerFail: BOOL;
    _Card1PrgTransferCmd: _sPRGTRANSFER_CMD;
    _DeviceOutHoldCfg: WORD;
    _EC_StatisticsLogCycleSec: UINT;
    _EC_StatisticsLogEnable: BOOL;
    _EIP_TDLinkStartCmd: BOOL;
    _EIP_TDLinkStopCmd: BOOL;
    _EIP1_TDLinkStartCmd: BOOL;
    _EIP1_TDLinkStopCmd: BOOL;
END_VAR

VAR_GLOBAL CONSTANT
    _AlarmFlag: WORD := 0;
    _BackupBusy: BOOL := FALSE;
    _Card1Access: BOOL := FALSE;
    _Card1Deteriorated: BOOL := FALSE;
    _Card1Err: BOOL := FALSE;
    _Card1PrgTransferSta: _sPRGTRANSFER_STA;
    _Card1Protect: BOOL;
    _Card1Ready: BOOL;
    _CJB_ErrSta: WORD;
    _CJB_MstrErrSta: WORD;
    _CJB_UnitErrSta: ARRAY[0..3, 0..9] OF WORD := [0(40)];
    _CurrentTime: DATE_AND_TIME;
    _DeviceOutHoldStatus: BOOL;
    _EC_CommErrTbl: ARRAY[1..192] OF BOOL := [0(192)];
    _EC_CycleExceeded: BOOL;
    _EC_DisableSlavTbl: ARRAY[1..192] OF BOOL := [0(192)];
    _EC_DisconnSlavTbl: ARRAY[1..192] OF BOOL := [0(192)];
    _EC_EntrySlavTbl: ARRAY[1..192] OF BOOL := [0(192)];
    _EC_ErrSta: WORD;
    _EC_InData1Invalid: BOOL;
    _EC_InData2Invalid: BOOL;
    _EC_InDataInvalid: BOOL;
    _EC_IndataInvalidErr: BOOL;
    _EC_LanHwErr: BOOL;
    _EC_LinkOffErr: BOOL;
    _EC_LinkStatus: BOOL;
    _EC_MacAdrErr: BOOL;
    _EC_MBXSlavTbl: ARRAY[1..192] OF BOOL := [0(192)];
    _EC_MsgErr: BOOL;
    _EC_MstrErr: WORD;
    _EC_NetCfgCmpErr: BOOL;
    _EC_NetCfgErr: BOOL;
    _EC_NetTopologyErr: BOOL;
    _EC_PDActive: BOOL;
    _EC_PDCommErr: BOOL;
    _EC_PDSendErr: BOOL;
    _EC_PDSlavTbl: ARRAY[1..192] OF BOOL := [0(192)];
    _EC_PDTimeoutErr: BOOL;
    _EC_PktMonStop: BOOL;
    _EC_PktSaving: BOOL;
    _EC_PortErr: WORD;
    _EC_RegSlavTbl: ARRAY[1..192] OF BOOL := [0(192)];
    _EC_SlavAdrDupErr: BOOL;
    _EC_SlavAppErr: BOOL;
    _EC_SlavEmergErr: BOOL;
    _EC_SlavErr: WORD;
    _EC_SlavErrTbl: ARRAY[1..192] OF BOOL := [0(192)];
    _EC_SlavInitErr: BOOL;
    _EC_StatisticsLogBusy: BOOL;
    _EC_StatisticsLogErr: BOOL;
    _EIP_BootpErr: BOOL;
    _EIP_CipErr: WORD;
    _EIP_DNSCfgErr: BOOL;
    _EIP_DNSSrvErr: BOOL;
    _EIP_ErrSta: WORD;
    _EIP_EstbTargetSta: ARRAY[0..255] OF BOOL := [0(256)];
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
    _EIP_RegTargetSta: ARRAY[0..255] OF BOOL := [0(256)];
    _EIP_TagAdrErr: BOOL;
    _EIP_TargetNodeErr: ARRAY[0..255] OF BOOL := [0(256)];
    _EIP_TargetPLCErr: ARRAY[0..255] OF BOOL := [0(256)];
    _EIP_TargetPLCModeSta: ARRAY[0..255] OF BOOL := [0(256)];
    _EIP_TcpAppCfgErr: BOOL;
    _EIP_TcpAppErr: WORD;
    _EIP_TDLinkAllRunSta: BOOL;
    _EIP_TDLinkCfgErr: BOOL;
    _EIP_TDLinkErr: BOOL;
    _EIP_TDLinkOpnErr: BOOL;
    _EIP_TDLinkRunSta: BOOL;
    _EIP1_BootpErr: BOOL;
    _EIP1_CipErr: WORD;
    _EIP1_EstbTargetSta: ARRAY[0..255] OF BOOL := [0(256)];
    _EIP1_EtnCfgErr: BOOL;
    _EIP1_EtnOnlineSta: BOOL;
    _EIP1_IdentityErr: BOOL;
    _EIP1_IPAdrCfgErr: BOOL;
    _EIP1_IPAdrDupErr: BOOL;
    _EIP1_LanHwErr: BOOL;
    _EIP1_MacAdrErr: BOOL;
    _EIP1_MultiSwONErr: BOOL;
    _EIP1_PortErr: WORD;
    _EIP1_RegTargetSta: ARRAY[0..255] OF BOOL := [0(256)];
    _EIP1_TagAdrErr: BOOL;
    _EIP1_TargetNodeErr: ARRAY[0..255] OF BOOL := [0(256)];
    _EIP1_TargetPLCErr: ARRAY[0..255] OF BOOL := [0(256)];
    _EIP1_TargetPLCModeSta: ARRAY[0..255] OF BOOL := [0(256)];
    _EIP1_TDLinkAllRunSta: BOOL;
    _EIP1_TDLinkCfgErr: BOOL;
    _EIP1_TDLinkErr: BOOL;
    _EIP1_TDLinkRunSta: BOOL;
    _ErrSta: WORD;
    _HardwareRevision: STRING[2] := '';

    // _MC_AX: ARRAY[0..15] OF _sAXIS_REF := [(
    //     Reserved:= [0(32)]
    // )];
    _MC_AX_ErrSta: ARRAY[0..15] OF WORD := [0(16)];
    _MC_COM: _sCOMMON_REF;
    _MC_ComErrSta: WORD;
    _MC_ErrSta: WORD;

    // _MC_GRP: ARRAY[0..7] OF _sGROUP_REF := [(
    //     Reserved:= [0(32)]
    // )];
    _MC_GRP_ErrSta: ARRAY[0..7] OF WORD := [0(8)];
    _NXB_ErrSta: WORD;
    _NXB_MaxUnitNo: UINT;
    _NXB_MstrErrSta: WORD;
    _NXB_UnitErrFlagTbl: ARRAY[0..8] OF BOOL := [0(9)];
    _NXB_UnitErrStaTbl: ARRAY[1..8] OF WORD := [0(8)];
    _NXB_UnitIOActiveTbl: ARRAY[0..8] OF BOOL := [0(9)];
    _NXB_UnitMsgActiveTbl: ARRAY[0..8] OF BOOL := [0(9)];
    _NXB_UnitRegTbl: ARRAY[0..8] OF BOOL := [0(7)];
    _PLC_ErrSta: WORD;

    // _PLC_OptBoardSta: ARRAY[1..2] OF _sOPTBOARD_STA := [(
    //     Run:= FALSE
    // )];
    _RetainFail: BOOL;
    _SelfTest_HighTemperature: BOOL;
    _SelfTest_LowBattery: BOOL;
    _SelfTest_LowFanRevolution: BOOL;
    _SystemSWVersion: ARRAY[0..2] OF USINT := [0(3)];
    _Unitversion: ARRAY[0..2] OF USINT := [0(3)];
END_VAR

//Built In Types

TYPE _sOPTSERIALERR_STA
    : STRUCT
        Error: BOOL;
        FramingErr: BOOL;
        OverrunErr: BOOL;
        ParityErr: BOOL;
        Reserve1: BOOL;
        Reserve2: BOOL;
    END_STRUCT
END_TYPE

TYPE _sPRGTRANSFER_CMD
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

TYPE _sPRGTRANSFER_STA
    : STRUCT
        Active: BOOL;
        Done: BOOL;
        Err: BOOL;
    END_STRUCT
END_TYPE

TYPE _sNTP_RESULT
    : STRUCT
        ExecNormal: BOOL;
        ExecTime: BOOL;
    END_STRUCT
END_TYPE

TYPE _sAXIS_REF
    : STRUCT
        Act: _sAXIS_REF_ACT_DATA;
        Cfg: _sAXIS_REF_CFG;
        Cmd: _sAXIS_REF_CMD_DATA;
        Details: _sAXIS_REF_DET;
        Dir: _sAXIS_REF_DIR;
        DrvStatus: _sAXIS_REF_STA_DRV;
        MFaultLvl: _sMC_REF_EVENT;
        Obsr: _sMC_REF_EVENT;
        Reserved: ARRAY[0..31] OF BYTE;
        Scale: _sAXIS_REF_SCALE;
        Status: _sAXIS_REF_STA;
    END_STRUCT
END_TYPE

TYPE _sAXIS_REF_ACT_DATA
    : STRUCT
        Pos: LREAL;
        TimeStamp: ULINT;
        Trq: LREAL;
        Vel: LREAL;
    END_STRUCT
END_TYPE

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

TYPE _eMC_AXIS_USE : 
    ( _mcNoneAxis, _mcUnusedAxis, _mcUsedAxis );
END_TYPE

TYPE _eMC_AXIS_TYPE : 
    (_mcServo, _mcEncdr, _mcVSrv, _mcVEnc);
END_TYPE


TYPE _sAXIS_REF_CMD_DATA
    : STRUCT
        AccDec: LREAL;
        Jerk: LREAL;
        Pos: LREAL;
        Trq: LREAL;
        Vel: LREAL;
    END_STRUCT
END_TYPE

TYPE _sAXIS_REF_DET
    : STRUCT
        Homed: BOOL;
        Idle: BOOL;
        InHome: BOOL;
        InPosWaiting: BOOL;
        Reserved: ARRAY[0..7] OF BYTE;
        VelLimit: BOOL;
    END_STRUCT
END_TYPE

TYPE _sAXIS_REF_DIR
    : STRUCT
        Posi: BOOL;
        Nega: BOOL;
    END_STRUCT
END_TYPE

TYPE _sAXIS_REF_STA_DRV
    : STRUCT
        CSP: BOOL;
        CST: BOOL;
        CSV: BOOL;
        DrvAlarm: BOOL;
        DrvWarning: BOOL;
        Home: BOOL;
        HomeSw: BOOL;
        ILA: BOOL;
        ImdStop: BOOL;
        Latch1: BOOL;
        Latch2: BOOL;
        MainPower: BOOL;
        N_OT: BOOL;
        P_OT: BOOL;
        Ready: BOOL;
        Reserved: ARRAY[0..7] OF BYTE;
        ServoOn: BOOL;
    END_STRUCT
END_TYPE

TYPE _sMC_REF_EVENT
    : STRUCT
        Active: BOOL;
        Code: WORD;        
    END_STRUCT
END_TYPE

TYPE _sAXIS_REF_SCALE
    : STRUCT
        CountMode: _eMC_COUNT_MODE;
        Den: LREAL;
        MaxPos: LREAL;
        MinPos: LREAL;
        Num: UDINT;
        Units: _eMC_UNITS;
    END_STRUCT
END_TYPE

TYPE _eMC_COUNT_MODE : 
    (_mcCountModeLinear, _mcCountModeRotary);
END_TYPE

TYPE _eMC_UNITS : 
    (_mcPls, _mcMm, _mcUm, _mcNm, _mcDeg, _mcInch);
END_TYPE

TYPE _sAXIS_REF_STA
    : STRUCT
        Continuous: BOOL;
        Coordinated: BOOL;
        Disabled: BOOL;
        Discrete: BOOL;
        ErrorStop: BOOL;
        Homing: BOOL;
        Ready: BOOL;
        Reserved: ARRAY[0..7] OF BYTE;
        Standstill: BOOL;
        Stopping: BOOL;
        Synchronized: BOOL;
    END_STRUCT
END_TYPE

TYPE _sCOMMON_REF
    : STRUCT
        Cfg: _sCOMMON_REF_CFG;
        MFaultLvl: _sMC_REF_EVENT;
        Obsvr: _sMC_REF_EVENT;
        PFaultLvl: _sMC_REF_EVENT;
        Reserved: ARRAY[0..31] OF BYTE;
        Status: _sCOMMON_REF_STA;
    END_STRUCT
END_TYPE

TYPE _sCOMMON_REF_CFG
    : STRUCT
        Reserved: ARRAY[0..31] OF BYTE;
    END_STRUCT
END_TYPE

TYPE _sCOMMON_REF_STA
    : STRUCT
        CamTableBusy: BOOL;
        GenerateCamBusy: BOOL;
        Reserved: ARRAY[0..15] OF BYTE;
        RunMode: BOOL;
        TestMode: BOOL;
    END_STRUCT
END_TYPE

TYPE _sGROUP_REF
    : STRUCT
        Cfg: _sGROUP_REF_CFG;
        Cmd: _sGROUP_REF_CMD_DATA;
        Details: _sGROUP_REF_DET;
        Kinematics: _sGROUP_REF_KIM;
        MFaultLvl: _sMC_REF_EVENT;
        Obsr: _sMC_REF_EVENT;
        Reserved: ARRAY[0..31] OF BYTE;
        Status: _sGROUP_REF_STA;
    END_STRUCT
END_TYPE

TYPE _sGROUP_REF_CFG
    : STRUCT
        ExecId: UINT;
        GrpEnable: _eMC_GROUP_USE;
        GrpNo: UINT;
        Reserved: ARRAY[0..7] OF BYTE;
    END_STRUCT
END_TYPE

TYPE _eMC_GROUP_USE : 
    (_mcNoneGroup, _mcUnusedGroup, _mcUsedGroup);
END_TYPE

TYPE _sGROUP_REF_CMD_DATA
    : STRUCT
        AccDec: LREAL;
        Vel: LREAL;
    END_STRUCT
END_TYPE

TYPE _sGROUP_REF_DET
    : STRUCT
        Idle: BOOL;
        InPosWaiting: BOOL;
        Reserved: ARRAY[0..7] OF BYTE;
    END_STRUCT
END_TYPE

TYPE _sGROUP_REF_KIM
    : STRUCT
        Axis: ARRAY[0..31] OF UINT;
        GrpType: _eMC_GROUP_TYPE;
        Reserved: ARRAY[0..13] OF BYTE;
    END_STRUCT
END_TYPE

TYPE _eMC_GROUP_TYPE :
    (_mcXY, _mcXYZ, _mcXYZU);
END_TYPE

TYPE _sGROUP_REF_STA
    : STRUCT
        Disabled: BOOL;
        ErrorStop: BOOL;
        Moving: BOOL;
        Ready: BOOL;
        Reserved: ARRAY[0..7] OF BYTE;
        Standby: BOOL;
        Stopping: BOOL;
    END_STRUCT
END_TYPE

TYPE _sOPTBOARD_STA
    : STRUCT
        Error: BOOL;
        IsDetect: BOOL;
        Run: BOOL;
    END_STRUCT
END_TYPE

TYPE _ePLC_UNIT_TYPE : 
    ( _CPU_UNIT );
END_TYPE

TYPE _sNXUNIT_ID 
    : STRUCT
        NodeAdr: UINT;
        IPAdr: ARRAY[0..55] OF BYTE;
        UnitNo: UDINT;
        Path: ARRAY[0..64] OF BYTE;
        PathLength: USINT;
    END_STRUCT
END_TYPE

//Built In Functions

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

FUNCTION GetNXBError : BOOL
    VAR_INPUT
        UnitProxy: _sNXUNIT_ID;
        Level: UINT;
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

FUNCTION PLC_ReadTotalPowerOnTime : TIME
    VAR_INPUT
        UnitType: _ePLC_UNIT_TYPE;
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
