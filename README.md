# stickle
Structured Text Unit Test examples for pipelines.

## Compiling Structured Text

For now, the rusty compiler only outputs Linux ELF binary format, not Windows PE format. All commands have to be run in a Linux environment.

### For Windows

- Install the Windows Subsystem for Linux (WSL).

- Install Docker Desktop using the WSL2 backend.

- In a WSL shell, install the Dotnet SDK:

    `sudo snap install --classic dotnet dotnet-runtime-100 dotnet-sdk-100`

## Compiling for a C# Unit Test

- Open this example project in the WSL.

- Run the following command from the root directory:

    ```
    docker run --rm --mount type=bind,src=./examples,dst=/examples --mount type=bind,src=./shared,dst=/shared ghcr.io/plc-lang/rusty:master "plc /examples/hello_world.st -o /shared/hello_world.so --shared --linker=cc --target=x86_64"
    ```

    - Ensure that Docker Desktop is running.

- Alternatively, Download and install [the build artifact](https://github.com/PLC-lang/rusty/actions/workflows/linux.yml) yourself. Each build pipeline for the Rusty Compiler produces a `plc` executable.

    ```
    plc /examples/hello_world.st -o /shared/hello_world.so --shared --linker=cc --target=x86_64
    ```

    - If you go this route you will have to keep your version up to date manually!

- Once your Structured Text is compiled, simply run `dotnet test` from the root directory.

## Further Reading

[Structured Text Compiler - Documentation](https://plc-lang.github.io/rusty/intro_1.html)

[Structured Text Compiler - Source Code](https://github.com/PLC-lang/rusty)

[Foreign Function Invocations](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke)
