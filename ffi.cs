using System.Runtime.InteropServices;
using System;

namespace stickle;

public partial class ClampAndSawFFI
{
    [DllImport("../../../shared/libclampandsaw", EntryPoint = "main", CallingConvention = CallingConvention.Cdecl)]
    public static extern int main();
}
