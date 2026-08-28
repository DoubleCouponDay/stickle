
TYPE XmlTestStruct
    : STRUCT
        Count: INT;
        Flag: BOOL;
        Label: STRING[16];
        Samples: ARRAY[0..3] OF DINT;
    END_STRUCT
END_TYPE


TYPE XmlTestGrid : ARRAY[0..1, 1..3] OF INT; END_TYPE


TYPE XmlTestAlias : DINT; END_TYPE


PROGRAM TestAllBuiltins
    VAR_EXTERNAL
        _PLC_OptSerialErrSta: ARRAY[1..2] OF _sOPTSERIALERR_STA;
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

    VAR
        time1: TIME;
        usint1: USINT;
        info1: UINT;
        dateandtime1: DATE_AND_TIME;
        bool1: BOOL;
        uint1: UINT;
        dword1: DWORD;
        udint1: UDINT;
        string1: STRING;
        sid1: _sNXUNIT_ID;
        test1: _sOPTSERIALERR_STA;
        sizedString1: STRING[32];
        wideString1: WSTRING[16];
        struct1: XmlTestStruct;
        grid1: XmlTestGrid;
        alias1: XmlTestAlias;
    END_VAR

    //builtin functions
    TraceSamp(usint1, usint1);
    TraceTrig(usint1);
    GetTraceStatus(usint1, bool1, bool1, bool1, bool1);
    SetAlarm(5, info1, info1);
    ResetAlarm(5);
    GetAlarm(uint1, uint1);
    GetPLCError(uint1, dword1);
    GetEIPError(uint1, dword1);
    GetMCError(uint1, dword1);
    GetECError(uint1, dword1);
    SetInfo(5, info1, info1);
    GetNTPStatus(dateandtime1, bool1);
    GetMyTaskStatus(time1, time1, time1, udint1, bool1, udint1);
    GetMyTaskInterval();
    Task_IsActive(string1);
    Lock(usint1);
    Unlock(usint1);
    ActEventTask(string1);
    Get10msClk();
    Get10msCnt();
    GetPrgHashCode(dword1, dword1, dword1, dword1);
END_PROGRAM
