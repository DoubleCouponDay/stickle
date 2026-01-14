using System.Runtime.InteropServices;
using System;

namespace stickle;

public partial class ClampAndSawFFI
{
    [DllImport("../../../compiled/lib_structured_text", EntryPoint = "PRG_ClampAndSaw", CallingConvention = CallingConvention.Cdecl)]
    public static extern int PRG_ClampAndSaw();

    [DllImport("../../../compiled/lib_structured_text", EntryPoint = "GetState", CallingConvention = CallingConvention.Cdecl)]
    public static extern int GetState();

    [DllImport("../../../compiled/lib_structured_text", EntryPoint = "SetThntd", CallingConvention = CallingConvention.Cdecl)]
    public static extern int SetThntd(bool input);

    [DllImport("../../../compiled/lib_structured_text", EntryPoint = "ResetState", CallingConvention = CallingConvention.Cdecl)]
    public static extern int ResetState();

    [DllImport("../../../compiled/lib_structured_text", EntryPoint = "test1", CallingConvention = CallingConvention.Cdecl)]
    public static extern int test1(bool input);
}
