cmake_minimum_required(VERSION 3.13...3.25)

add_executable(${CMAKE_PROJECT_NAME})

target_sources(${CMAKE_PROJECT_NAME}
    PRIVATE steganography.cc
)

target_link_libraries(${PROJECT_NAME}
    PRIVATE util
)

install(TARGETS ${PROJECT_NAME}
    RUNTIME DESTINATION ${STEG_BIN_DIR}
)
