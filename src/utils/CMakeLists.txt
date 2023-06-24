cmake_minimum_required(VERSION 3.11...3.25)

find_package(PNG REQUIRED)
find_package(JPEG REQUIRED)
find_package(Boost REQUIRED)

project(util DESCRIPTION "steganography utility functions"
             LANGUAGES   CXX
)

add_library(${PROJECT_NAME} STATIC)

target_sources(${PROJECT_NAME}
    PRIVATE steganography_util.cc
)

target_include_directories(${PROJECT_NAME}
    PUBLIC ${STEG_INCLUDE_DIR}
)

target_link_libraries(${PROJECT_NAME}
    PRIVATE JPEG::JPEG
    PRIVATE PNG::PNG
    PRIVATE Boost::boost
)

