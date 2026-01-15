
plc ./source/* -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.o

if($LASTEXITCODE -ne 0) {
    exit
}

clang ./compiled/lib_structured_text.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./compiled/lib_structured_text.dll

if($LASTEXITCODE -ne 0) {
    exit
}

plc ./source/clampandsaw.st --xml-omron -i ./source/externals.st -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.xml
