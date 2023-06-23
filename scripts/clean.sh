#!/bin/bash

source config.sh

# Remove the binary directory.
if [ -d $STEG_BIN_DIR ]
then
    echo "removing '$STEG_BIN_DIR'"
    rm -r $STEG_BIN_DIR
fi

# Remove the CMake build directory.
if [ -d $STEG_BUILD_DIR ]
then
    echo "removing '$STEG_BUILD_DIR'"
    rm -r $STEG_BUILD_DIR
fi
