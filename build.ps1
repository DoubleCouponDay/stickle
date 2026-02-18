#libomron
plc ./libomron/libomron.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/libomron.o

if($LASTEXITCODE -ne 0) {
    exit
}

clang ./compiled/libomron.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:libomron/exports.def" -o ./compiled/libomron.dll

if($LASTEXITCODE -ne 0) {
    exit
}

# clampandsaw
plc ./source/* -c -l iec61131std -l libomron -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.o

if($LASTEXITCODE -ne 0) {
    exit
}

clang ./compiled/lib_structured_text.o --shared -l iec61131std -l libomron -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./compiled/lib_structured_text.dll

if($LASTEXITCODE -ne 0) {
    exit
}

plc ./source/clampandsaw.st --xml-omron -i ./source/externals.st -i ./source/omron.st -l iec61131std -l libomron -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.xml
