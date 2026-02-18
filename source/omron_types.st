
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
