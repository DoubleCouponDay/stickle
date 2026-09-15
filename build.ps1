# libbuiltins
plc ./libbuiltins/*.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/libbuiltins.o

if($LASTEXITCODE -ne 0) {
    exit
}

clang ./compiled/libbuiltins.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:libbuiltins/exports.def" -o ./compiled/libbuiltins.dll

if($LASTEXITCODE -ne 0) {
    exit
}

# clampandsaw
plc ./source/*.st -c --generate-external-constructors -i ./externals/stdlib_externals.st -i ./libNX1P2/externals/nx1p2_externals.st -L ./compiled -l iec61131std -l libbuiltins -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.o

if($LASTEXITCODE -ne 0) {
    exit
}

clang ./compiled/lib_structured_text.o --shared -l iec61131std -L ./compiled -l libbuiltins -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./compiled/lib_structured_text.dll

if($LASTEXITCODE -ne 0) {
    exit
}

plc ./source/clampandsaw.st ./source/testallbuiltins.st --xml-omron --generate-external-constructors -i ./externals/stdlib_externals.st -i ./libNX1P2/externals/nx1p2_externals.st -L ./compiled -l iec61131std -l libbuiltins -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.xml
