# Stickle
Example Structured Text project for plain-text source control and CICD development using the [Rusty Compiler](https://github.com/PLC-lang/rusty).

## Setting up the Development Environment

### Compiling on Windows

- Install [`LLVM 14.0.6`](https://github.com/PLC-lang/llvm-package-windows/releases/tag/v14.0.6) and add it's `bin` folder to your `PATH` environment variable.

- Download `plc.zip` from the [Windows Build Pipeline](https://github.com/PLC-lang/rusty/actions/workflows/windows.yml).

    - Add it's location to the PATH environment variable. An AppData location is recommended.

- Download `stdlib.lib` from the same pipeline and install it to the same folder.

- Install the `Windows SDK` and `MSVC`. You can use the Visual Studio Installer to do this or install them as standalone packages. 

- Create a `LIB` environment variable containing paths to `libiec61131std.lib`, `ws2_32.lib`, `ntdll.lib`, `userenv.lib`, `libcmt.lib`, `oldnames.lib` and `libucrt.lib`.

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
    plc ./examples/clampandsaw.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/libclampandsaw.o
    clang ./compiled/libclampandsaw.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./compiled/libclampandsaw.dll
    ```

- Finally, link everything together and execute the unit test:

    ```
    dotnet test
    ```

### Compiling on Linux

- Install the following packages:

    ```
    sudo apt update && sudo apt install build-essential
    sudo snap install --classic dotnet && sudo snap install dotnet-sdk-100 && sudo snap install dotnet-runtime-100 && sudo snap install aspnetcore-runtime-100
    ```

- Download `plc.zip` from the [Linux Build Pipeline](https://github.com/PLC-lang/rusty/actions/workflows/linux.yml) and decompress it into the `/usr/bin`. Each build pipeline for the Rusty Compiler produces a `plc` executable.

    ```
    cd /mnt/c/Users/<USERNAME>
    7z e ./Downloads/plc.zip
    sudo cp ./Downloads/plc /usr/bin
    ```

- Download `stdlib.zip` from the [Linux Build Pipeline](https://github.com/PLC-lang/rusty/actions/workflows/linux.yml) and decompress it.

    Ensure you take the `libiec61131std.so` file that corresponds with your microprocessor architecture (most likely x86_64-linux-gnu).

    ```
    sudo cp ~/Downloads/stdlib/x86_64-linux-gnu/libiec61131std.so /usr/lib
    ```
    
- Run the compilation:
    
    ```
    plc ./examples/clampandsaw.st --shared --linker=cc --target=x86_64 -l iec61131std -o ./compiled/libclampandsaw.so
    ```

- Link the dynamic library with dotnet and execute the unit test:

    ```
    dotnet test
    ```

## Compiling with Docker

- Install Docker Desktop using the WSL2 backend.

- Download the compiler image:

    ```
    docker pull ghcr.io/plc-lang/rusty:master
    ```

- Run the following command from the root directory:

    ```
    docker run --rm --mount type=bind,src=./examples,dst=/examples --mount type=bind,src=./compiled,dst=/compiled --mount type=bind,src=/lib,dst=/copiedlibs ghcr.io/plc-lang/rusty:master "plc /examples/clampandsaw.st --linker=cc --target=x86_64 -L /copiedlibs -l iec61131std --shared -o /compiled/libclampandsaw.so"
    ```

## Compiling as a standalone executable

This can be useful if you need to debug a ST file and log directly to console using the `puts` or `printf` function. Please note that for whatever reason, `printf` does not accept CRLF or LF as newline sequences. Use `printf` to append to the current line and `puts` to write a new line.

```
plc /examples/clampandsaw.st --linker=cc --target=x86_64 -l iec61131std -o /compiled/clampandsaw
```

To execute the standalone program:

```
./clampandsaw
```

## Further Reading

[Rusty Compiler - Documentation](https://plc-lang.github.io/rusty/intro_1.html)

[Rusty Compiler - Community](https://github.com/PLC-lang/rusty)

[Foreign Function Invocations](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke)

## FAQ - Frequently Asked Questions

### Why use Docker Desktop?

Docker provides a package manager for retrieving the latest updates to the Rusty Compiler. You can download it from the Linux Build Pipeline but you will need to maintain that installation manually. With Docker, simply run `docker pull ghcr.io/plc-lang/rusty:master` to retrieve the latest updates.

`stdlib` still needs to be maintained manually but the compiler maintainer has signalled, in future, it will be bundled into the docker image.

## TODO

- Propertly implement the converter into the Rusty Compiler.

- Develop the example unit test further.
