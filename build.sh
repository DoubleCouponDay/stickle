#!/usr/bin/env bash

# libNX1P2
plc ./libNX1P2/*.st --shared --linker=cc -l iec61131std -o ./compiled/libNX1P2.so

if [ $? -ne 0 ]; then
    exit 1
fi

# clampandsaw
plc ./source/*.st --shared --linker=cc --generate-external-constructors -i ./externals/stdlib_externals.st -i ./externals/omron_externals.st -L ./compiled -l iec61131std -l NX1P2 --linker-arg=--rpath='$ORIGIN' -o ./compiled/lib_structured_text.so

if [ $? -ne 0 ]; then
    exit 1
fi

plc ./source/clampandsaw.st ./source/testallbuiltins.st --xml-omron --generate-external-constructors -i ./externals/stdlib_externals.st -i ./externals/omron_externals.st -L ./compiled -l iec61131std -l NX1P2 -o ./compiled/lib_structured_text.xml

if [ $? -ne 0 ]; then
    exit 1
fi
