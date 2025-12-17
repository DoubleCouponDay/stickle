# stickle
Structured Text Unit Test examples for pipelines.

## Compiling Structured Text

### For Windows

- Install Docker Desktop using the WSL2 backend (Windows Subsystem for Linux)

    - For now, the rusty compiler only outputs Linux ELF binary format, not Windows PE format.

    - When using the WSL, ensure that Docker Desktop is running.

- In the WSL install the Dotnet SDK:

    `sudo snap install --classic dotnet dotnet dotnet-runtime-100 dotnet-sdk-100`

## Testing linking with a C# Unit Test

- Open this example project in the WSL.

- Run the following command from the root directory:

```
docker run --rm --mount type=bind,src=./examples,dst=/examples --mount type=bind,src=./shared,dst=/shared ghcr.io/plc-lang/rusty:master "plc /examples/hello_world.st -o /shared/hello_world.so --shared --linker=cc --target=x86_64"
```

Simply run `dotnet test` from the root directory.

## Further Reading

[Structured Text Compiler - Documentation](https://plc-lang.github.io/rusty/intro_1.html)

[Structured Text Compiler - Source Code](https://github.com/PLC-lang/rusty)

[Foreign Function Invocations](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke)
