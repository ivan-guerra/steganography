cmake_minimum_required(VERSION 3.13...3.25)

project(steganography
    DESCRIPTION "hide one image within another"
)

set(STEG_BIN_DIR "${CMAKE_SOURCE_DIR}/bin"
    CACHE STRING "${PROJECT_NAME} binary directory.")
set(STEG_INCLUDE_DIR "${CMAKE_SOURCE_DIR}/include"
    CACHE STRING     "${PROJECT_NAME} include directory.")

add_subdirectory(src)
