# libbug
plc ./bug/libbug.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/libbug.o

if($LASTEXITCODE -ne 0) {
    exit
}

clang ./compiled/libbug.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:bug/exports.def" -o ./compiled/libbug.dll #must be dll

if($LASTEXITCODE -ne 0) {
    exit
}

# bugrecreation
plc ./bug/recreation.st -c -l iec61131std -l libbug -l ws2_32 -l ntdll -l userenv -o ./compiled/recreation.o

if($LASTEXITCODE -ne 0) {
    exit
}

clang ./compiled/recreation.o --shared -l iec61131std -l libbug -l ws2_32 -l ntdll -l userenv -o ./compiled/recreation.dll #must be dll

if($LASTEXITCODE -ne 0) {
    exit
}

plc ./bug/recreation.st --xml-omron -l iec61131std -l libbug -l ws2_32 -l ntdll -l userenv -o ./compiled/recreation.xml
