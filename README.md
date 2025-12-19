# stickle
Example Structured Text project for plain-text source control and CICD development.

## Compiling Structured Text

For now, the rusty compiler only outputs Linux ELF binary format, not Windows PE format. All commands have to be run in a Linux environment.

### For Windows

- Install the Windows Subsystem for Linux (WSL).

- Install Docker Desktop using the WSL2 backend.

- In a WSL shell, install the Dotnet SDK:

    `sudo snap install --classic dotnet`

    `sudo snap install dotnet-sdk-100`

    `sudo snap install dotnet-runtime-100`

    `sudo snap install aspnetcore-runtime-100`

- Download the compiler image:

    `docker pull ghcr.io/plc-lang/rusty:master`

- Download `stdlib` from the [Linux Build Pipeline](https://github.com/PLC-lang/rusty/actions/workflows/linux.yml) and export it into the `/lib` folder/.

    Ensure you export the `libiec61131std.so` file that corresponds with your microprocessor architecture (most likely x86_64-linux-gnu).

    `sudo cp ./libiec61131std.so /lib`

## Compiling for a C# Unit Test

- Open this example project in the WSL.

- Download and install [the build artifact](https://github.com/PLC-lang/rusty/actions/workflows/linux.yml) yourself. Each build pipeline for the Rusty Compiler produces a `plc` executable.

    ```
    plc /examples/clampandsaw.st --shared --linker=cc --target=x86_64 -o /compiled/libclampandsaw.so 
    ```

- Once your Structured Text is compiled, simply run `dotnet test` from the root directory.

## Compiling with Docker

- Ensure that Docker Desktop is running.

- Run the following command from the root directory:

    ```
    docker run --rm --mount type=bind,src=./examples,dst=/examples --mount type=bind,src=./compiled,dst=/compiled --mount type=bind,src=/lib,dst=/copiedlibs ghcr.io/plc-lang/rusty:master "plc /examples/clampandsaw.st --linker=cc --target=x86_64 -L /copiedlibs -l iec61131std --shared -o /compiled/libclampandsaw.so"
    ```

## Compiling as a standalone executable

This can be useful if you need to debug a ST file and log directly to console using the `puts` or `printf` function. Please note that for whatever reason, `printf` does not accept CRLF or LF as newline sequences. Use `printf` to append to the current line and `puts` to write a new line.

```
docker run --rm --mount type=bind,src=./examples,dst=/examples --mount type=bind,src=./compiled,dst=/compiled --mount type=bind,src=/lib,dst=/copiedlibs ghcr.io/plc-lang/rusty:master "plc /examples/clampandsaw.st --linker=cc --target=x86_64 -L /copiedlibs -l iec61131std -o /compiled/clampandsaw"
```

To execute the standalone program:

```
./clampandsaw
```

## Further Reading

[Structured Text Compiler - Documentation](https://plc-lang.github.io/rusty/intro_1.html)

[Structured Text Compiler - Source Code](https://github.com/PLC-lang/rusty)

[Foreign Function Invocations](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke)

## FAQ - Frequently Asked Questions

### Why do you need a WSL Environment?

The WSL allows a Windows-based development team to use the ELF format `.so` dynamic libraries and link them with a Dotnet Core application. The Rusty compiler does have a Windows build pipeline available but it cannot yet compile to a Windows PE format `.dll` Dynamically Linked Libray.

### Why use Docker Desktop?

Docker provides a package manager for retrieving the latest updates to the Rusty Compiler. You can download it from the Linux Build Pipeline but you will need to maintain that installation manually. With Docker, simply run `docker pull ghcr.io/plc-lang/rusty:master` to retrieve the latest updates.

`stdlib` still needs to be maintained manually but the compiler engineer has signalled in future it will be bundled into the docker image.

## TODO

- Develop the example unit test further.

- Develop rough glue code for converting ST to Omron XML.

- Propertly implement the converter into the Rusty Compiler.

- test Windows development with the Clang linker.

    `choco install llvm`

- Make documentation PR for windows development.

- Maybe clean up all the warnings when compiling stdlib from source.
