using System.Runtime.InteropServices;
using System;

namespace stickle;

public partial class HelloWorldFFI
{
    [LibraryImport("../../../shared/hello_world.so", StringMarshalling = StringMarshalling.Utf16, SetLastError = true)]
    public static partial int main();
}

public partial class ClampAndSawFFI
{
    [LibraryImport("../../../shared/clampandsaw.so", StringMarshalling = StringMarshalling.Utf16, SetLastError = true)]
    public static partial int main();

    [DllImport("../../../lib/libiec61131std.a", EntryPoint = "DINT_TO_STRING", CallingConvention = CallingConvention.Cdecl)]
    public static extern string DINT_TO_STRING(uint input);
}
