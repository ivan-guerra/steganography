#!/bin/bash

CWD=$(pwd)

# Root directory.
STEG_PROJECT_PATH=$(dirname ${CWD})

# Scripts directory.
STEG_SCRIPTS_PATH="${STEG_PROJECT_PATH}/scripts"

# Binary directory.
STEG_BIN_DIR="${STEG_PROJECT_PATH}/bin"

# CMake build files and cache.
STEG_BUILD_DIR="${STEG_PROJECT_PATH}/build"
