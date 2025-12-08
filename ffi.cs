using System.Runtime.InteropServices;
using System;

namespace stickle;

public partial class FFI {
    [LibraryImport("hello_world.so", StringMarshalling = StringMarshalling.Utf16, SetLastError = true)]
    public static partial int main();
}
