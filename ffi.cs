using System.Runtime.InteropServices;
using System;

namespace stickle;

public partial class ClampAndSawFFI
{
    [DllImport("../../../compiled/lib_structured_text", EntryPoint = "main", CallingConvention = CallingConvention.Cdecl)]
    public static extern int ExecuteStateMachine();
}
