cmake_minimum_required(VERSION 3.20)
project(Callback C CXX)

set(TARGET callback)

include_directories(${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_CURRENT_BINARY_DIR})

set(TARGET_SRCS
    callback.c
)

add_library(${TARGET} SHARED ${TARGET_SRCS})

set_target_properties(${TARGET} PROPERTIES
    POSITION_INDEPENDENT_CODE ON
)

install(TARGETS ${TARGET}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
)

if (WIN32)
    target_link_libraries(${TARGET} PRIVATE ws2_32)
    target_compile_definitions(${TARGET} PRIVATE -DBUILDING_DLL)
    set_target_properties(${TARGET} PROPERTIES
        WINDOWS_EXPORT_ALL_SYMBOLS ON
    )
endif()

target_compile_features(${TARGET} PRIVATE cxx_std_11)
