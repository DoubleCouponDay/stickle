#!/usr/bin/env bash

# libbuiltins
plc './libbuiltins/**/*.st' --shared --linker=cc -L ./compiled -l iec61131std -o ./compiled/libbuiltins.so

if [ $? -ne 0 ]; then
    exit 1
fi

# clampandsaw
plc './source/**/*.st' --shared --linker=cc --generate-external-constructors -i ./externals/stdlib_externals.st -i ./libNX1P2/externals/nx1p2_externals.st -L ./compiled -l iec61131std -l builtins --linker-arg=--rpath='$ORIGIN' -o ./compiled/lib_structured_text.so

if [ $? -ne 0 ]; then
    exit 1
fi

plc ./source/clampandsaw.st ./source/testallbuiltins.st --xml-omron --generate-external-constructors -i ./externals/stdlib_externals.st -i ./libNX1P2/externals/nx1p2_externals.st -L ./compiled -l iec61131std -l builtins -o ./compiled/lib_structured_text.xml

if [ $? -ne 0 ]; then
    exit 1
fi
