using System.Runtime.InteropServices;
using System;

namespace stickle;

public partial class ClampAndSawFFI
{
    [DllImport("../../../compiled/lib_structured_text", EntryPoint = "main", CallingConvention = CallingConvention.Cdecl)]
    public static extern int ExecuteStateMachine();

    [DllImport("../../../compiled/lib_structured_text", EntryPoint = "Set_THNTD", CallingConvention = CallingConvention.Cdecl)]
    public static extern int Set_THNTD(bool input);

    [DllImport("../../../compiled/lib_structured_text", EntryPoint = "ResetState", CallingConvention = CallingConvention.Cdecl)]
    public static extern int ResetState();

}
