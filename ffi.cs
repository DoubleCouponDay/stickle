
using System.Runtime.InteropServices;
using System.Security.Authentication.ExtendedProtection;

[DllImport("hello_world.so")]
static extern int helloworld_main();
