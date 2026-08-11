cmake_minimum_required(VERSION 3.20)
project(fluvio_client_cpp VERSION 0.1.0 LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

option(FLUVIO_CLIENT_CPP_BUILD_TESTS "Build the fluvio_client_cpp test executables" ON)

if(FLUVIO_CLIENT_CPP_BUILD_TESTS)
    enable_testing()
endif()

# ---------------------------------------------------------------------------
# Rust static library
# ---------------------------------------------------------------------------
# Map the CMake build type onto a cargo profile so `cmake --build . --config
# Release` (or -DCMAKE_BUILD_TYPE=Release) produces an optimized Rust lib
# instead of always building `debug`.
if(CMAKE_BUILD_TYPE STREQUAL "Release" OR
        CMAKE_BUILD_TYPE STREQUAL "RelWithDebInfo" OR
        CMAKE_BUILD_TYPE STREQUAL "MinSizeRel")
    set(FLUVIO_CARGO_PROFILE_DIR "release")
    set(FLUVIO_CARGO_FLAGS --release)
else()
    set(FLUVIO_CARGO_PROFILE_DIR "debug")
    set(FLUVIO_CARGO_FLAGS "")
endif()

set(RUST_TARGET_DIR "${CMAKE_CURRENT_SOURCE_DIR}/target/${FLUVIO_CARGO_PROFILE_DIR}")
set(FLUVIO_STATIC_LIB "${RUST_TARGET_DIR}/libfluvio_client_cpp.a")

# Run cargo once at configure time so the cxxbridge-generated headers exist
# and can be located below. Errors here now fail the configure step instead
# of silently producing a broken target.
message(STATUS "Building Rust static library (${FLUVIO_CARGO_PROFILE_DIR}) ...")
execute_process(
        COMMAND cargo build ${FLUVIO_CARGO_FLAGS}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        RESULT_VARIABLE FLUVIO_CARGO_BUILD_RESULT
)
if(NOT FLUVIO_CARGO_BUILD_RESULT EQUAL 0)
    message(FATAL_ERROR "cargo build failed (exit code ${FLUVIO_CARGO_BUILD_RESULT})")
endif()

if(NOT EXISTS "${FLUVIO_STATIC_LIB}")
    message(FATAL_ERROR "Expected Rust static library not found at: ${FLUVIO_STATIC_LIB}")
endif()

# Locate the cxxbridge-generated headers. Globbing for '*lib.rs.h' can match
# more than one file if stale build artifacts from a previous crate version
# are still sitting under target/, so fail loudly instead of silently
# picking a possibly-wrong match with list(GET ... 0 ...).
file(GLOB_RECURSE CXXBRIDGE_HEADERS "${RUST_TARGET_DIR}/build/*lib.rs.h")
file(GLOB_RECURSE CXXBRIDGE_SOURCES_ALL "${RUST_TARGET_DIR}/build/*lib.rs.cc")
file(GLOB_RECURSE CXX_CORE_HEADER "${RUST_TARGET_DIR}/build/*/cxx.h")

list(LENGTH CXXBRIDGE_HEADERS _n_headers)
list(LENGTH CXXBRIDGE_SOURCES_ALL _n_sources)
list(LENGTH CXX_CORE_HEADER _n_core_headers)

if(_n_headers EQUAL 0 OR _n_sources EQUAL 0 OR _n_core_headers EQUAL 0)
    message(FATAL_ERROR
            "Could not find cxxbridge-generated files under ${RUST_TARGET_DIR}/build. "
            "Try removing the target/ directory and re-running cmake."
    )
endif()

if(_n_headers GREATER 1 OR _n_sources GREATER 1 OR _n_core_headers GREATER 1)
    message(FATAL_ERROR
            "Found multiple candidate cxxbridge build directories under "
            "${RUST_TARGET_DIR}/build (stale artifacts from a previous build?). "
            "Run 'cargo clean' and re-configure.\n"
            "Headers found: ${CXXBRIDGE_HEADERS}"
    )
endif()

list(GET CXXBRIDGE_SOURCES_ALL 0 CXXBRIDGE_SOURCES)
list(GET CXXBRIDGE_HEADERS 0 CXXBRIDGE_HEADER)
get_filename_component(CXXBRIDGE_HDR_DIR ${CXXBRIDGE_HEADER} DIRECTORY)
list(GET CXX_CORE_HEADER 0 CXX_CORE_HDR)
get_filename_component(CXX_CORE_DIR ${CXX_CORE_HDR} DIRECTORY)
get_filename_component(CXX_ROOT_DIR ${CXX_CORE_DIR} DIRECTORY)

# Re-run cargo (and thus cmake) automatically whenever Rust sources change,
# so incremental `cmake --build .` picks up Rust-side edits without needing
# a manual reconfigure.
file(GLOB_RECURSE FLUVIO_RUST_SOURCES CONFIGURE_DEPENDS
        "${CMAKE_CURRENT_SOURCE_DIR}/src/*.rs"
)
if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml")
    list(APPEND FLUVIO_RUST_SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml")
endif()

add_custom_command(
        OUTPUT "${FLUVIO_STATIC_LIB}.stamp"
        COMMAND cargo build ${FLUVIO_CARGO_FLAGS}
        COMMAND ${CMAKE_COMMAND} -E touch "${FLUVIO_STATIC_LIB}.stamp"
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        DEPENDS ${FLUVIO_RUST_SOURCES}
        COMMENT "Rebuilding Rust static library (${FLUVIO_CARGO_PROFILE_DIR})"
        VERBATIM
)
add_custom_target(fluvio_client_cpp_rust DEPENDS "${FLUVIO_STATIC_LIB}.stamp")

set(GENERATED_INCLUDE_DIR ${CMAKE_CURRENT_BINARY_DIR}/generated_include/fluvio_client_cpp)
file(MAKE_DIRECTORY ${GENERATED_INCLUDE_DIR}/rust)

configure_file(
        ${CXXBRIDGE_HEADER}
        ${GENERATED_INCLUDE_DIR}/lib.rs.h
        COPYONLY
)

configure_file(
        ${CXX_CORE_HDR}
        ${GENERATED_INCLUDE_DIR}/rust/cxx.h
        COPYONLY
)

# ---------------------------------------------------------------------------
# Library target
# ---------------------------------------------------------------------------
add_library(fluvio_client_cpp STATIC ${CXXBRIDGE_SOURCES})
add_library(ASTRAOS::fluvio_client_cpp ALIAS fluvio_client_cpp)
add_dependencies(fluvio_client_cpp fluvio_client_cpp_rust)

find_package(Threads REQUIRED)

target_link_libraries(fluvio_client_cpp
        PUBLIC
        Threads::Threads
        ${CMAKE_DL_LIBS}
        INTERFACE
        $<BUILD_INTERFACE:${FLUVIO_STATIC_LIB}>
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_LIBDIR}/libfluvio_client_cpp.a>
)

target_include_directories(fluvio_client_cpp
        PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_BINARY_DIR}/generated_include>
        $<INSTALL_INTERFACE:include>
        PRIVATE
        ${CXXBRIDGE_HDR_DIR}/../..
        ${CXX_ROOT_DIR}
)

# ---------------------------------------------------------------------------
# Tests
# ---------------------------------------------------------------------------
if(FLUVIO_CLIENT_CPP_BUILD_TESTS)
    function(fluvio_add_test _name _source)
        add_executable(${_name} ${_source})
        target_link_libraries(${_name} PRIVATE fluvio_client_cpp)
        add_test(NAME ${_name} COMMAND ${_name})
    endfunction()

    fluvio_add_test(test_producer tests/test_producer.cpp)
    fluvio_add_test(test_consumer tests/test_consumer.cpp)
    fluvio_add_test(test_admin    tests/test_admin.cpp)
    fluvio_add_test(test_config   tests/test_config.cpp)
    fluvio_add_test(test_c        tests/test_c.c)
    fluvio_add_test(test_auth     tests/test_auth.cpp)
    fluvio_add_test(test_auth_c   tests/test_auth_c.c)
endif()

# ---------------------------------------------------------------------------
# Installation / find_package() support
# ---------------------------------------------------------------------------
include(GNUInstallDirs)
include(CMakePackageConfigHelpers)

install(DIRECTORY
        include/
        DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)

install(DIRECTORY
        ${CMAKE_CURRENT_BINARY_DIR}/generated_include/
        DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)

install(FILES
        ${FLUVIO_STATIC_LIB}
        DESTINATION ${CMAKE_INSTALL_LIBDIR}
)

install(TARGETS fluvio_client_cpp
        EXPORT fluvio_client_cppTargets
        ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
        LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
        RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
        INCLUDES DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)

install(EXPORT fluvio_client_cppTargets
        FILE fluvio_client_cppTargets.cmake
        NAMESPACE ASTRAOS::
        DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/fluvio_client_cpp
)

# Config file consumers use via find_package(fluvio_client_cpp CONFIG REQUIRED)
configure_package_config_file(
        cmake/fluvio_client_cppConfig.cmake.in
        ${CMAKE_CURRENT_BINARY_DIR}/fluvio_client_cppConfig.cmake
        INSTALL_DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/fluvio_client_cpp
)

# Version file so consumers can pin/require compatible versions, e.g.
# find_package(fluvio_client_cpp 0.1 REQUIRED)
write_basic_package_version_file(
        ${CMAKE_CURRENT_BINARY_DIR}/fluvio_client_cppConfigVersion.cmake
        VERSION ${PROJECT_VERSION}
        COMPATIBILITY SameMajorVersion
)

install(FILES
        ${CMAKE_CURRENT_BINARY_DIR}/fluvio_client_cppConfig.cmake
        ${CMAKE_CURRENT_BINARY_DIR}/fluvio_client_cppConfigVersion.cmake
        DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/fluvio_client_cpp
)