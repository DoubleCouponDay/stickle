{namespace := 'Common'}

(* Air Cylinder Actual Values *)
TYPE AirCylinderDevActual
    : STRUCT
        ExtendTime: DINT; // Time to Extend in ms (if has limits)
        RetractTime: DINT; // Time to Retract in ms (if has limits)
    END_STRUCT
END_TYPE

(* Air Cylinder Device Commands *)
TYPE AirCylinderDevCommand
    : STRUCT
        Extend: BOOL; // Extend Cylinder
        Retract: BOOL; // Retract Cylinder
        Release: BOOL; // Release Cylinder (if valve allows)
        Reset: BOOL; // Reset Fault
    END_STRUCT
END_TYPE

TYPE AirCylinderDevFlag
    : STRUCT
        PartialExtend: BOOL; // Allow extend without checking for extend limit
        PartialRetract: BOOL; // Allow retract without checking for retract limit
        DisableOvershoot: BOOL; // Disable Overshoot error during partial extend/retract
        Simulate: BOOL;
    END_STRUCT
END_TYPE

(* Air Cylinder Device Parameters *)
TYPE AirCylinderDevParameter
    : STRUCT
        ExtendTime: INT; // Max Extend Time, ms, 500, 0, 2000
        RetractTime: INT; // Max Retract Time, ms, 500, 0, 2000
        OutputPulseTime: INT; // Output Pulse Time, ms, 100, 0, 10000
        HasExtendLimit: BOOL; // Has Extended Limit, , true
        HasRetractLimit: BOOL; // Has Retracted Limit, , true
    END_STRUCT
END_TYPE

(* Air Cylinder Device Statuses *)
TYPE AirCylinderDevStatus
    : STRUCT
        OK: BOOL; // Cylinder is Healthy (NOT Fault)
        Configured: BOOL; // Necessary Parameters Are Set
        Extended: BOOL; // Cylinder is Extended
        Retracted: BOOL; // Cylinder is Retracted
        Moving: BOOL; // Cylinder is Moving
    END_STRUCT
END_TYPE

(* Air Cylinder Device Errors *)
TYPE AirCylinderDevError
    : STRUCT
        ExtendFail: BOOL; // Failed to Extend
        RetractFail: BOOL; // Failed to Retract
        LimitFault: BOOL; // Limit Switch Fault
        Overshoot: BOOL; // Cylinder Has Overshot
    END_STRUCT
END_TYPE

(* Air Cylinder Device *)
TYPE AirCylinderDev
    : STRUCT
        Command: AirCylinderDevCommand;
        Flag: AirCylinderDevFlag;
        Parameter: AirCylinderDevParameter;
        Status: AirCylinderDevStatus;
        Actual: AirCylinderDevActual;
        Error: AirCylinderDevError;
    END_STRUCT
END_TYPE

TYPE AirCylinderCyc
    : STRUCT
        Status: AirCylinderDevStatus;
        Actual: AirCylinderDevActual;
        Error: AirCylinderDevError;
    END_STRUCT
END_TYPE

TYPE AnalogSensorDevActual
    : STRUCT
        Value: LREAL;
        Raw: LREAL;
    END_STRUCT
END_TYPE

TYPE AnalogSensorDevParameter
    : STRUCT
        RawLower: INT; // Raw Lower, , 0, 0, 32767
        RawUpper: INT; // Raw Upper, , 32767, 0, 32767
        ValueLower: LREAL; // Value Lower, units, 0
        ValueUpper: LREAL; // Value Upper, units, 100
        Offset: LREAL; // Value Offset, units, 0
    END_STRUCT
END_TYPE

TYPE AnalogSensorDev
    : STRUCT
        Parameter: AnalogSensorDevParameter;
        Actual: AnalogSensorDevActual;
    END_STRUCT
END_TYPE

TYPE AnalogSensorCyc
    : STRUCT
        Actual: AnalogSensorDevActual;
    END_STRUCT
END_TYPE

(* A point in 2D space [x,y] *)
TYPE Coord
    : STRUCT
        X: LREAL;
        Y: LREAL;
    END_STRUCT
END_TYPE

TYPE DOLDevActual
    : STRUCT
        CurrentRunTime: DINT;
    END_STRUCT
END_TYPE

TYPE DOLDevCommand
    : STRUCT
        Run: BOOL;
        Stop: BOOL;
    END_STRUCT
END_TYPE

TYPE DOLDevStatus
    : STRUCT
        Running: BOOL;
        OK: BOOL;
    END_STRUCT
END_TYPE

TYPE DOLDevFlag
    : STRUCT
        Enable: BOOL;
    END_STRUCT
END_TYPE

TYPE DOLDevTarget
    : STRUCT
        RunTime: DINT;
    END_STRUCT
END_TYPE

TYPE DOLDev
    : STRUCT
        Actual: DOLDevActual;
        Command: DOLDevCommand;
        Flag: DOLDevFlag;
        Status: DOLDevStatus;
        Target: DOLDevTarget;
    END_STRUCT
END_TYPE

TYPE DOLCyc
    : STRUCT
        Actual: DOLDevActual;
        Status: DOLDevStatus;
    END_STRUCT
END_TYPE

TYPE VSDDevCommsOut
    : STRUCT
        RunForward: BOOL;
        RunReverse: BOOL;
        FaultReset: BOOL;
        Acceleration: REAL;
        Deceleration: REAL;
        Frequency: REAL;
    END_STRUCT
END_TYPE

TYPE VSDDevCommsIn
    : STRUCT
        RunningForward: BOOL;
        RunningReverse: BOOL;
        Fault: BOOL;
        Warning: BOOL;
        Remote: BOOL;
        CommsError: BOOL;
        AtSpeed: BOOL;
        Frequency: REAL;
    END_STRUCT
END_TYPE

TYPE VSDDevCommand
    : STRUCT
        RunForward: BOOL;
        RunReverse: BOOL;
        Stop: BOOL;
        Reset: BOOL;
    END_STRUCT
END_TYPE

TYPE VSDDevFlag
    : STRUCT
        Enable: BOOL;
        Simulate: BOOL;
    END_STRUCT
END_TYPE

TYPE VSDDevStatus
    : STRUCT
        OK: BOOL; // Drive is Healthy (NOT Fault)
        Warning: BOOL; // Drive has a Warning
        Running: BOOL; // Drive is Running
        LocalControl: BOOL; // Drive is being controlled Locally (from unit)
        AtSpeed: BOOL; // Drive is at speed (i.e. FreqRef = FreqOut)
        RunningForward: BOOL; // Drive is Running Forward
        RunningReverse: BOOL; // Drive is Running Reverse
        Configured: BOOL;
    END_STRUCT
END_TYPE

TYPE VSDDevParameter
    : STRUCT
        DefaultAcceleration: INT; // Default Acceleration Time, ms, 5000, 0, 60000
        DefaultDeceleration: INT; // Default Deceleration Time, ms, 5000, 0, 60000
        DefaultSpeed: LREAL; // Default Speed, %, 100, 0, 100
        MaxFrequency: LREAL; // Max Frequency, Hz, 50, 1, 120
        ReverseDirection: BOOL; // Reverse Direction, , false
    END_STRUCT
END_TYPE

TYPE VSDDevTarget
    : STRUCT
        RunTime: DINT; // ms
        Acceleration: INT; // ms
        Deceleration: INT; // ms
        Speed: LREAL; // %
    END_STRUCT
END_TYPE

TYPE VSDDevActual
    : STRUCT
        Speed: LREAL; // %
        CurrentRunTime: DINT; // ms
    END_STRUCT
END_TYPE

TYPE VSDDevError
    : STRUCT
        CommsFail: BOOL;
    END_STRUCT
END_TYPE

TYPE VSDDev
    : STRUCT
        Command: VSDDevCommand;
        Flag: VSDDevFlag;
        Target: VSDDevTarget;
        Parameter: VSDDevParameter;
        Status: VSDDevStatus;
        Actual: VSDDevActual;
        Error: VSDDevError;
    END_STRUCT
END_TYPE

TYPE VSDCyc
    : STRUCT
        Status: VSDDevStatus;
        Actual: VSDDevActual;
        Error: VSDDevError;
    END_STRUCT
END_TYPE

TYPE DigitalSensorDevStatus
    : STRUCT
        Active: BOOL; // Sensor is On
    END_STRUCT
END_TYPE

TYPE DigitalSensorDevParameter
    : STRUCT
        DebounceTime: INT; // Debounce Time, ms, 0, 0, 1000
        ActiveLow: BOOL; // Active Low, false
    END_STRUCT
END_TYPE

TYPE DigitalSensorDevFlag
    : STRUCT
        Simulate: BOOL;
        SimulatedInput: BOOL;
    END_STRUCT
END_TYPE

TYPE DigitalSensorDev
    : STRUCT
        Flag: DigitalSensorDevFlag;
        Parameter: DigitalSensorDevParameter;
        Status: DigitalSensorDevStatus;
    END_STRUCT
END_TYPE

TYPE DigitalSensorCyc
    : STRUCT
        Status: DigitalSensorDevStatus;
    END_STRUCT
END_TYPE

TYPE HydraulicCylinderDevActual
    : STRUCT
        ExtendTime: DINT; // Time to Extend in ms 
        RetractTime: DINT; // Time to Retract in ms
        Position: LREAL; // Position of Cylinder in mm
    END_STRUCT
END_TYPE

TYPE HydraulicCylinderDevCommand
    : STRUCT
        Extend: BOOL; // Extend Cylinder
        Retract: BOOL; // Retract Cylinder
        Release: BOOL; // Release Cylinder
        Reset: BOOL; // Reset Fault
    END_STRUCT
END_TYPE

TYPE HydraulicCylinderDevError
    : STRUCT
        ExtendFail: BOOL; // Failed to Extend
        RetractFail: BOOL; // Failed to Retract
    END_STRUCT
END_TYPE

TYPE HydraulicCylinderDevFlag
    : STRUCT
        Simulate: BOOL;
        PartialExtend: BOOL;
        PartialRetract: BOOL;
        SimulateExtend: BOOL;
        SimulateRetract: BOOL;
    END_STRUCT
END_TYPE

TYPE HydraulicCylinderDevParameter
    : STRUCT
        MaxLimit: LREAL; // Max Limit, mm, 100, -100, 1000
        MinLimit: LREAL; // Min Limit, mm, 0, -100, 1000
        MaxExtendTime: INT; // Max Extend Time, ms, 500, 0, 2000
        MaxRetractTime: INT; // Max Retract Time, ms, 500, 0, 2000
        InPositionTolerance: LREAL; //  In Position tolerance, mm , 0, -10, 10
        ReverseDirection: BOOL; // Reverse Cylinder Direction, , false
        HasExtendLimit: BOOL; // Has Extended Limit, , true
        HasRetractLimit: BOOL; // Has Retrracted Limit, , true
        HasPositionSensor: BOOL; // Has Position Sensor, , false
    END_STRUCT
END_TYPE

TYPE HydraulicCylinderDevStatus
    : STRUCT
        OK: BOOL;
        Configured: BOOL;
        Retracted: BOOL;
        Extended: BOOL;
        Moving: BOOL;
    END_STRUCT
END_TYPE

TYPE HydraulicCylinderDev
    : STRUCT
        Command: HydraulicCylinderDevCommand;
        Flag: HydraulicCylinderDevFlag;
        Parameter: HydraulicCylinderDevParameter;
        Status: HydraulicCylinderDevStatus;
        Actual: HydraulicCylinderDevActual;
        Error: HydraulicCylinderDevError;
    END_STRUCT
END_TYPE

TYPE HydraulicCylinderCyc
    : STRUCT
        Status: HydraulicCylinderDevStatus;
        Actual: HydraulicCylinderDevActual;
        Error: HydraulicCylinderDevError;
    END_STRUCT
END_TYPE

TYPE LogLevelEnum
    : (
        LogLevelTrace := 0,
        LogLevelDebug := 1,
        LogLevelInfo := 2,
        LogLevelWarn := 3,
        LogLevelError := 4,
        LogLevelFatal := 5
    );
END_TYPE

TYPE LogEntry
    : STRUCT
        Level: INT;
        Timestamp: DATE_AND_TIME;
        Description: STRING[256];
    END_STRUCT
END_TYPE

TYPE Log
    : STRUCT
        Buffer: ARRAY[0..99] OF LogEntry;
        CurrentElements: UINT;
        MaxElements: UINT;
        ReadRequest: BOOL;
        ReadsConfirmed: INT;
    END_STRUCT
END_TYPE

TYPE FlexxPumpLubricatorDevCommand
    : STRUCT
        DispensePk1: BOOL;
        DispensePk2: BOOL;
        DispensePk1Pk2: BOOL;
        FIL: BOOL;
        Reset: BOOL;
    END_STRUCT
END_TYPE

TYPE FlexxPumpLubricatorDevParameter
    : STRUCT
        FILDelay: INT;
        DispenseDelay: INT;
    END_STRUCT
END_TYPE

TYPE FlexxPumpLubricatorDevStatus
    : STRUCT
        Busy: BOOL;
        Empty: BOOL;
        Error: BOOL;
        Ready: BOOL;
    END_STRUCT
END_TYPE

TYPE FlexxPumpLubricatorDev
    : STRUCT
        Command: FlexxPumpLubricatorDevCommand;
        Parameter: FlexxPumpLubricatorDevParameter;
        Status: FlexxPumpLubricatorDevStatus;
    END_STRUCT
END_TYPE

TYPE MachineModCommand
    : STRUCT
        ResetAll: BOOL;
        PLCReset: BOOL;
        DriveReset: BOOL;
        CylinderReset: BOOL;
    END_STRUCT
END_TYPE

TYPE MachineModErrorCode
    : STRUCT
        PLCError: DINT;
        MCError: DINT;
        EtherCATError: DINT;
        EtherCATFaultAtNode: DINT;
    END_STRUCT
END_TYPE

TYPE MachineModStatus
    : STRUCT
        SafetyOK: BOOL; // EStops and Safety Devices Healthy
        PLCOK: BOOL;
        MCOK: BOOL;
        EtherCATOK: BOOL;
        AirPressureOK: BOOL; // Air Pressure and Pneumatic Devices Healthy
        DevicesOK: BOOL; // Servos and Drives Healthy
    END_STRUCT
END_TYPE

TYPE MachineModState
    : STRUCT
        Estopped: BOOL;
        Initialising: BOOL;
        Healthy: BOOL;
        Faulted: BOOL;
    END_STRUCT
END_TYPE

TYPE MachineMod
    : STRUCT
        Status: MachineModStatus;
        State: MachineModState;
        ErrorCode: MachineModErrorCode;
        Command: MachineModCommand;
    END_STRUCT
END_TYPE

TYPE MachineCyc
    : STRUCT
        State: MachineModState;
        Status: MachineModStatus;
        ErrorCode: MachineModErrorCode;
    END_STRUCT
END_TYPE

TYPE ModStatus
    : STRUCT
        OK: BOOL;
        Configured: BOOL;
    END_STRUCT
END_TYPE

TYPE SafetyDevStatus
    : STRUCT
        OK: BOOL;
    END_STRUCT
END_TYPE

TYPE SafetyDev
    : STRUCT
        Status: SafetyDevStatus;
    END_STRUCT
END_TYPE

TYPE ServoDevErrorCode
    : STRUCT
        Axis: INT;
        Drive: INT;
    END_STRUCT
END_TYPE

TYPE ServoDevFlag
    : STRUCT
        Enable: BOOL;
        Simulate: BOOL;
        DisableSWLimits: BOOL;
    END_STRUCT
END_TYPE

TYPE ServoDevCommand
    : STRUCT
        Move: BOOL;
        MoveVelocity: BOOL;
        TorqueControl: BOOL;
        Stop: BOOL;
        ImmediateStop: BOOL;
        Reset: BOOL;
        ResetFollowingError: BOOL;
        Couple: BOOL;
        Uncouple: BOOL;
        Home: BOOL;
    END_STRUCT
END_TYPE

TYPE ServoDevParameter
    : STRUCT
        DefaultVelocity: LREAL; // Default Velocity, mm/s, 100, 0
        DefaultAcceleration: LREAL; // Default Acceleration, mm/s/s, 100, 0
        DefaultJerk: LREAL; // Default Jerk, mm/s/s/s, 0 , 0
        MinimumPosition: LREAL; // Minimum Position, mm
        MaximumPosition: LREAL; // Maximum Position, mm
        HomeVelocity: LREAL; // Home Velocity, mm/s, 10, 0
        HomeOffset: LREAL; // Home Offset, mm, 0, 0
        FollowingErrorLimit: LREAL; // Following Error Limit, mm, 0, 0
        TorqueLimit: LREAL; // Torque Limit, %, 0, 0, 300
        CalibrationRatio: LREAL; // Calibration Ratio, mm/rev, 0, 0
        DefaultTorque: LREAL; // Default Torque, %, 0, 0,1000
        DefaultTorqueRamp: LREAL; // Default Torque Ramp, %/s, 0, 0
    END_STRUCT
END_TYPE

TYPE ServoDevStatus
    : STRUCT
        PowerOn: BOOL;
        OK: BOOL;
        Configured: BOOL;
        Homing: BOOL;
        Homed: BOOL;
        Moving: BOOL;
        InPosition: BOOL;
        AtTorque: BOOL; // TRUE when at Target Torque, only during a Torque Control command
        AtVelocity: BOOL; // TRUE when at Target Velocity, only during a Velocity Move command
        Coupled: BOOL;
        InternalLimit: BOOL;
    END_STRUCT
END_TYPE

TYPE ServoDevError
    : STRUCT
        CommsFail: BOOL;
        PowerFail: BOOL;
        PositionError: BOOL;
        CommandAborted: BOOL;
        CommandError: BOOL;
        MCFault: BOOL;
        MinLimit: BOOL;
        MaxLimit: BOOL;
    END_STRUCT
END_TYPE

TYPE ServoDevActual
    : STRUCT
        Position: LREAL;
        Torque: LREAL;
        Velocity: LREAL;
    END_STRUCT
END_TYPE

TYPE ServoDevTarget
    : STRUCT
        Position: LREAL;
        Velocity: LREAL; // A negative velocity will move in reverse direction when using MoveVelocity command
        Torque: LREAL;
        Acceleration: LREAL;
        TorqueRamp: LREAL;
        Deceleration: LREAL;
        Jerk: LREAL;
    END_STRUCT
END_TYPE

TYPE ServoDev
    : STRUCT
        Command: ServoDevCommand;
        Flag: ServoDevFlag;
        Target: ServoDevTarget;
        Parameter: ServoDevParameter;
        Status: ServoDevStatus;
        Actual: ServoDevActual;
        Error: ServoDevError;
        ErrorCode: ServoDevErrorCode;
    END_STRUCT
END_TYPE

TYPE ServoCyc
    : STRUCT
        Status: ServoDevStatus;
        Actual: ServoDevActual;
        Error: ServoDevError;
        ErrorCode: ServoDevErrorCode;
    END_STRUCT
END_TYPE

TYPE SequencerModCommand
    : STRUCT
        Start: BOOL; // Starts the Sequencer step execution
        Pause: BOOL; // Pauses the Sequencer after the current step has executed
        StopReset: BOOL; // Resets the Sequencer from Faulted to Ready state or from Running to Ready
        Skip: BOOL;
        Resume: BOOL;
        Fault: BOOL; // Puts the Sequencer in the Faulted state
    END_STRUCT
END_TYPE

TYPE SequencerModDesc
    : STRUCT
        SeqStep: STRING[256]; // Description of the current step action
        SeqWait: STRING[256]; // Description of the transition conditions for the current step
        State: STRING[256]; // Description of the current State
        Fault: STRING[256]; // Description when Faulted
    END_STRUCT
END_TYPE

TYPE SequencerModState
    : STRUCT
        Ready: BOOL;
        Running: BOOL;
        Paused: BOOL;
        Faulted: BOOL;
    END_STRUCT
END_TYPE

TYPE SequencerModFlag
    : STRUCT
        SeqDataRequest: BOOL; // Requests the HMI to send new SequenceData
        PrintLabelsRequest: BOOL; // Requests the HMI to print labels
        CanSkip: BOOL; // Enables the skip button on the HMI
        CanResume: BOOL; // Enables the Resume button on the HMI
    END_STRUCT
END_TYPE

TYPE SequencerMod
    : STRUCT
        Command: SequencerModCommand;
        Flag: SequencerModFlag;
        Desc: SequencerModDesc;
        State: SequencerModState;
        CutIndex: INT; // Index for the SeqData.Cuts array
    END_STRUCT
END_TYPE

TYPE SequencerCyc
    : STRUCT
        State: SequencerModState;
        Desc: SequencerModDesc;
    END_STRUCT
END_TYPE

(* The dimensions of a piece of timber *)
TYPE TimberStock
    : STRUCT
        Width: LREAL; // Width refers to the longer edge, i.e. 90mm in 90x35
        Depth: LREAL; // Depth refers to the shorter edge, i.e. 30mm in 90x35
        Length: LREAL; // Length of the timber
        DensityFactor: LREAL; // Density factor to scale speed, must be >= 1
        Species: STRING[256]; // Timber Species
        Grade: STRING[256]; // Timber Grade
        Treatment: STRING[256]; // Timber Treatment
    END_STRUCT
END_TYPE

TYPE CalibrationModCommand
    : STRUCT
        Start: BOOL;
        Resume: BOOL;
        Reset: BOOL;
    END_STRUCT
END_TYPE

TYPE CalibrationModParameter
    : STRUCT
        Param1: LREAL;
        Param2: LREAL;
        Param3: LREAL;
        Param4: LREAL;
        Param5: LREAL;
    END_STRUCT
END_TYPE

TYPE CalibrationModStatus
    : STRUCT
        Running: BOOL;
        Waiting: BOOL;
        Done: BOOL;
    END_STRUCT
END_TYPE

TYPE CalibrationMod
    : STRUCT
        Command: CalibrationModCommand;
        Status: CalibrationModStatus;
        Parameter: CalibrationModParameter;
        SeqSelect: INT; // To select between multiple sequences
    END_STRUCT
END_TYPE

TYPE CalibrationCyc
    : STRUCT
        Status: CalibrationModStatus;
        SeqSelect: INT; // To select between multiple sequences
    END_STRUCT
END_TYPE

FUNCTION WriteToLog : BOOL
    VAR_INPUT
        Message: STRING[256];
        Level: LogLevelEnum;
        CallSite: STRING[256];
    END_VAR

    VAR_IN_OUT
        LogStack: Log;
    END_VAR
END_FUNCTION
