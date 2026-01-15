# Stickle
Example Structured Text project for plain-text source control and CICD development using the [Rusty Compiler](https://github.com/doublecouponday/rusty-fork).

## Setting up the Development Environment

### Compiling on Windows

- Install [`LLVM 14.0.6`](https://github.com/PLC-lang/llvm-package-windows/releases/tag/v14.0.6) and add it's `bin` folder to your `PATH` environment variable.

- Install `plc.zip` from the [Windows Build Pipeline](https://github.com/doublecouponday/rusty-fork/actions/workflows/windows.yml).

    - Add it's location to the PATH environment variable. An AppData location is recommended for installation.

- Install `stdlib.lib` from the same pipeline and install it to the same folder.

- Install the `Windows SDK` and `MSVC`. You can use the Visual Studio Installer to do this or install them as standalone packages. 

- Create a `LIB` environment variable containing paths to `iec61131std.lib`, `ws2_32.lib`, `ntdll.lib`, `userenv.lib`, `libcmt.lib`, `oldnames.lib` and `libucrt.lib`.

    - Your environment variable should look something like this:

    ```
    C:/Users/<USERNAME>/AppData/Local/rustycompiler;
    C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x64;
    C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\ucrt\x64;
    C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.43.34808\lib\x64;
    ```

- Restart your terminals to refresh the environment.

- Proceed with compilation:
    
    ```
    plc ./source/* -c -i ./lib/externals.st -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.o

    clang ./compiled/lib_structured_text.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./compiled/lib_structured_text.dll
    ```

- Finally, link everything together and execute the unit test:

    ```
    dotnet test
    ```

You can perform this compilation procedure by running the Powershell script, instead.

```
./build.ps1
```

Structured Text can also be compiled to IEC 61131-10 XML, which imports into Omron Sysmac Studio.

```
plc ./source/clampandsaw.st --xml-omron -i ./lib/externals.st -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.xml
```

### Compiling on Linux

- Install the following packages:

    ```
    sudo apt update && sudo apt install build-essential

    sudo snap install --classic dotnet && sudo snap install dotnet-sdk-100 && sudo snap install dotnet-runtime-100 && sudo snap install aspnetcore-runtime-100
    ```

- Download `plc.zip` from the [Linux Build Pipeline](https://github.com/doublecouponday/rusty-fork/actions/workflows/linux.yml) and decompress it into the `/usr/bin`. Each build pipeline for the Rusty Compiler produces a `plc` executable.

    ```
    cd /mnt/c/Users/<USERNAME>

    7z e ./Downloads/plc.zip

    sudo cp ./Downloads/plc /usr/bin
    ```

- Download `stdlib.zip` from the [Linux Build Pipeline](https://github.com/doublecouponday/rusty-fork/actions/workflows/linux.yml) and decompress it into the `/lib` folder.

    Ensure you take the `libiec61131std.so` file that corresponds with your microprocessor architecture (most likely x86_64-linux-gnu).

    ```
    sudo cp ~/Downloads/stdlib/x86_64-linux-gnu/libiec61131std.so /lib
    ```
    
- Run the compilation:
    
    ```
    plc ./source/clampandsaw.st --shared --linker=cc --target=x86_64 -l iec61131std -o ./compiled/lib_structured_text.so
    ```

You can perform this compilation procedure by running the Bash script, instead.

```
./build.sh
```

Link the dynamic library with dotnet and execute the unit test:

```
dotnet test
```

## Compiling with Docker

- Install Docker Desktop using the WSL2 backend and ensure it is running.

- On Windows, run this:

```
docker run --rm --mount type=bind,src=./source,dst=/source --mount type=bind,src=./compiled,dst=/compiled --mount type=bind,src=C:/Users/Sam/AppData/Local/rustycompiler,dst=/copiedlibs  ghcr.io/doublecouponday/rusty:master "plc /source/* --linker=cc --target=x86_64 -L /copiedlibs -l iec61131std --shared -o /compiled/lib_structured_text.dll"
```

- On Linux, run this: 

```
docker run --rm --mount type=bind,src=./source,dst=/source --mount type=bind,src=./compiled,dst=/compiled --mount type=bind,src=/lib,dst=/copiedlibs  ghcr.io/doublecouponday/rusty:master "plc /source/* --linker=cc --target=x86_64 -L /copiedlibs -l iec61131std --shared -o /compiled/l
ib_structured_text.so"
```

## Compiling as a standalone executable

This can be useful if you need to debug a ST file and log directly to console using the `puts` or `printf` function. Please note that for whatever reason, `printf` does not accept CRLF or LF as newline sequences. Use `printf` to append to the current line and `puts` to write a new line.

Windows:
```
plc ./source/clampandsaw.st -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/clampandsaw
```

Linux:
```
plc /source/clampandsaw.st --linker=cc --target=x86_64 -l iec61131std -o /compiled/clampandsaw
```

To execute the standalone program:

```
./clampandsaw
```

## Further Reading

[Rusty Compiler - Documentation](https://plc-lang.github.io/rusty/intro_1.html)

[Rusty Compiler - Community](https://github.com/doublecouponday/rusty-fork)

[Foreign Function Invocations](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke)

## FAQ - Frequently Asked Questions

### Why use Docker Desktop?

Docker provides a package manager for retrieving the latest updates to the Rusty Compiler. You can download it from Github Build Pipeline but you will need to maintain that installation manually. With Docker, simply run `docker pull ghcr.io/doublecouponday/rusty:master` to retrieve the latest updates.

`stdlib` still needs to be maintained manually but the compiler maintainer has signalled, in future, it will be bundled into the docker image.

### Why did you convert the Rusty Xml Nodes to all take String instead of &str?

InitialValue AstNodes needed to be converted to Strings at runtime. Those strings must pass ownership to the Node Tree which won't work when passing references. The lifetime of those &strs would have been greater than their owned Strings.

### Why did you change all the Xml Nodes children fields to Vec\<Box\<dyn IntoNode\>\>?

I couldn't create a collection of Vec\<dyn IntoNode\> because their size is not known at compile time. This prevented me from iteratively adding to a Vec of child nodes, then using the .children() function.

## TODO

- Fix no External References created in Functions.

## Nice to have

- clean up all .is_none() calls and bubble up Err results to the closure, without escaping the for loop.

    should try to minimize unwrap calls.

- see if HashSets can be cleaned up by using &String instead of String.

- Support Unions.

- Somehow add semicolons to the end of every statement block? Rusty doesn't validate that.

- Somehow filter Function Block instances as function internal or as global. They should be TODO

- Support network publish modes for globals. eg: custom token..

- Global pattern input files for xml conversion.

    This works for `-c` so why not other compilation modes?

- Support Variable comments.

- stdlib bundled inside docker image.

- actions, classes, methods, init functions, project init functions.

- add lit tests for Windows.

- Bring DLLs up to feature parity with LIBs.

- Remove the tabs in the first indentation column.

    This is due to copying from source code which has an indentation on the first column.

- Support Omron Builtin Global Variables.

- Print a warning on missing semicolons?
