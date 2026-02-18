#!/usr/bin/env bash

# libomron
plc ./libomron/libomron.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/libomron.o

if [ $? -ne 0 ]; then
    exit 1
fi

clang ./compiled/libomron.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:libomron/exports.def" -o ./compiled/libomron.dll

if [ $? -ne 0 ]; then
    exit 1
fi

# clampandsaw
plc ./source/* -c -l iec61131std -l libomron -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.o

if [ $? -ne 0 ]; then
    exit 1
fi

clang ./compiled/lib_structured_text.o --shared -l iec61131std -l libomron -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./compiled/lib_structured_text.dll

if [ $? -ne 0 ]; then
    exit 1
fi

plc ./source/clampandsaw.st ./source/builtins_test.st --xml-omron -i ./source/externals.st -i ./source/omron_functions.st -i ./source/omron_types.st -i ./source/omron_vars.st -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.xml
