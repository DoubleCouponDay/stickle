# stickle
Structured Text Unit Test examples for pipelines.

## Compiling Structured Text

Install Docker and run the following command:

```
sudo docker run --rm --mount type=bind,src=./examples,dst=/examples --mount type=bind,src=./shared,dst=/shared ghcr.io/plc-lang/rusty:master "plc /examples/hello_world.st -o /shared/hello_world.so --shared --linker=cc --target=x86_64"
```

## Usage

```
dotnet test
```

## Further Reading

[Structured Text Compiler](https://plc-lang.github.io/rusty/intro_1.html)

[Foreign Function Invocations](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke)
