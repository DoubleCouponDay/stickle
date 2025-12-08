using System.Runtime.InteropServices;

namespace stickle;

public class FFI {
    [DllImport("../../../shared/hello_world.so")]
    public static extern int helloworld_main();
}
