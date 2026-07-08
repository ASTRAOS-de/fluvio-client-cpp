cmake_minimum_required(VERSION 3.20)
project(fluvio_client_cpp CXX C)

set(CMAKE_CXX_STANDARD 17)

# Enable tasting
enable_testing()

# Normally you would build Rust via FetchContent / Corrosion,
# But since 'cargo build' generates the static library, we'll assume it exists or run it.
execute_process(
    COMMAND cargo build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

set(RUST_TARGET_DIR "target/debug")
file(GLOB_RECURSE CXXBRIDGE_HEADERS "${RUST_TARGET_DIR}/build/*lib.rs.h")
file(GLOB_RECURSE CXXBRIDGE_SOURCES_ALL "${RUST_TARGET_DIR}/build/*lib.rs.cc")
file(GLOB_RECURSE CXX_CORE_HEADER "${RUST_TARGET_DIR}/build/*/cxx.h")
file(GLOB_RECURSE RUST_STATIC_LIB "${RUST_TARGET_DIR}/build/*/out/libfluvio*.a")
list(GET RUST_STATIC_LIB 0 FLUVIO_STATIC_LIB)

list(GET CXXBRIDGE_SOURCES_ALL 0 CXXBRIDGE_SOURCES)
list(GET CXXBRIDGE_HEADERS 0 CXXBRIDGE_HEADER)
get_filename_component(CXXBRIDGE_HDR_DIR ${CXXBRIDGE_HEADER} DIRECTORY)
list(GET CXX_CORE_HEADER 0 CXX_CORE_HDR)
get_filename_component(CXX_CORE_DIR ${CXX_CORE_HDR} DIRECTORY)
get_filename_component(CXX_ROOT_DIR ${CXX_CORE_DIR} DIRECTORY)

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

add_library(fluvio_client_cpp STATIC ${CXXBRIDGE_SOURCES})
add_library(ASTRAOS::fluvio_client_cpp ALIAS fluvio_client_cpp)

target_include_directories(fluvio_client_cpp
        PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<BUILD_INTERFACE:${GENERATED_INCLUDE_DIR}>
        $<INSTALL_INTERFACE:include>

        PRIVATE
        ${CXXBRIDGE_HDR_DIR}/../..
        ${CXX_ROOT_DIR}
)

# Producer Test
add_executable(test_producer tests/test_producer.cpp)
target_link_libraries(test_producer PRIVATE fluvio_client_cpp)
add_test(NAME fluvio_producer_test COMMAND test_producer)

# Consumer Test
add_executable(test_consumer tests/test_consumer.cpp)
target_link_libraries(test_consumer PRIVATE fluvio_client_cpp)
add_test(NAME fluvio_consumer_test COMMAND test_consumer)

# Admin Test
add_executable(test_admin tests/test_admin.cpp)
target_link_libraries(test_admin PRIVATE fluvio_client_cpp)
add_test(NAME fluvio_admin_test COMMAND test_admin)

# Config Test
add_executable(test_config tests/test_config.cpp)
target_link_libraries(test_config PRIVATE fluvio_client_cpp)
add_test(NAME fluvio_config_test COMMAND test_config)

# Pure C Native Test
add_executable(test_c tests/test_c.c)
target_include_directories(test_c PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(test_c PRIVATE
    ${FLUVIO_STATIC_LIB}
    stdc++ pthread dl m
)
add_test(NAME fluvio_c_native_test COMMAND test_c)

# Auth E2E CXX Test
add_executable(test_auth tests/test_auth.cpp)
target_link_libraries(test_auth PRIVATE fluvio_client_cpp)
add_test(NAME fluvio_auth_test COMMAND test_auth)

# Auth E2E C Native Test
add_executable(test_auth_c tests/test_auth_c.c)
target_include_directories(test_auth_c PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(test_auth_c PRIVATE
    ${FLUVIO_STATIC_LIB}
    stdc++ pthread dl m
)
add_test(NAME fluvio_auth_c_test COMMAND test_auth_c)

# Installation
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
)

install(EXPORT fluvio_client_cppTargets
    FILE fluvio_client_cppTargets.cmake
    NAMESPACE ASTRAOS::
    DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/fluvio_client_cpp
)

configure_package_config_file(
    cmake/fluvio_client_cppConfig.cmake.in
    ${CMAKE_CURRENT_BINARY_DIR}/fluvio_client_cppConfig.cmake
    INSTALL_DESTINATION
        ${CMAKE_INSTALL_LIBDIR}/cmake/fluvio_client_cpp
)

install(FILES
    ${CMAKE_CURRENT_BINARY_DIR}/fluvio_client_cppConfig.cmake
    DESTINATION
        ${CMAKE_INSTALL_LIBDIR}/cmake/fluvio_client_cpp
)

