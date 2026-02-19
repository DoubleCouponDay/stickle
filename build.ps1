# libomron
plc ./libomron/*.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/libNX1P2.o

if($LASTEXITCODE -ne 0) {
    exit
}

clang ./compiled/libNX1P2.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:libomron/exports.def" -o ./compiled/libNX1P2.dll

if($LASTEXITCODE -ne 0) {
    exit
}

# clampandsaw
plc ./source/*.st -c -i ./externals/stdlib_externals.st -i ./externals/omron_externals.st -l iec61131std -l libNX1P2 -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.o

if($LASTEXITCODE -ne 0) {
    exit
}

clang ./compiled/lib_structured_text.o --shared -l iec61131std -l libNX1P2 -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./compiled/lib_structured_text.dll

if($LASTEXITCODE -ne 0) {
    exit
}

plc ./source/clampandsaw.st ./source/testallbuiltins.st --xml-omron -i ./externals/stdlib_externals.st -i ./externals/omron_externals.st -l iec61131std -l libNX1P2 -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.xml
