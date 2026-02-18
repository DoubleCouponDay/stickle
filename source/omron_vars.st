//builtin variables

VAR_GLOBAL
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

    _MC_AX: ARRAY[0..15] OF _sAXIS_REF := [(
        Reserved:= [0(32)]
    )];
    _MC_AX_ErrSta: ARRAY[0..15] OF WORD := [0(16)];
    _MC_COM: _sCOMMON_REF;
    _MC_ComErrSta: WORD;
    _MC_ErrSta: WORD;

    _MC_GRP: ARRAY[0..7] OF _sGROUP_REF := [(
        Reserved:= [0(32)]
    )];
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

    _PLC_OptBoardSta: ARRAY[1..2] OF _sOPTBOARD_STA := [(
        Run:= FALSE
    )];
    _RetainFail: BOOL;
    _SelfTest_HighTemperature: BOOL;
    _SelfTest_LowBattery: BOOL;
    _SelfTest_LowFanRevolution: BOOL;
    _SystemSWVersion: ARRAY[0..2] OF USINT := [0(3)];
    _Unitversion: ARRAY[0..2] OF USINT := [0(3)];
END_VAR
