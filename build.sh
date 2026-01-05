#!/usr/bin/env bash

# Compile PLC sources to object file
plc ./source/* -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./compiled/lib_structured_text.o
if [ $? -ne 0 ]; then
    exit 1
fi

# Link object file into a shared DLL using clang and lld-link
clang ./compiled/lib_structured_text.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./compiled/lib_structured_text.dll

if [ $? -ne 0 ]; then
    exit 1
fi
