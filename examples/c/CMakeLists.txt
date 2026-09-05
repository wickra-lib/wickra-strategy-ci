# Build and run the C and C++ examples against the wickra-strategy-ci C ABI.
#
#   cargo build --release -p wickra-strategy-ci-c
#   cmake -S examples/c -B examples/c/build
#   cmake --build examples/c/build --config Release
#   ctest --test-dir examples/c/build -C Release --output-on-failure
#
# WICKRA_STRATEGY_CI_LIB_DIR defaults to the workspace target/release directory
# and can be overridden for out-of-tree builds.

cmake_minimum_required(VERSION 3.15)
project(wickra_strategy_ci_examples C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(WICKRA_STRATEGY_CI_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release"
    CACHE PATH "Directory containing the built wickra-strategy-ci C ABI library")

set(WICKRA_STRATEGY_CI_INCLUDE "${CMAKE_CURRENT_SOURCE_DIR}/../../bindings/c/include")

find_library(WICKRA_STRATEGY_CI_LIB
    NAMES wickra_strategy_ci libwickra_strategy_ci
    HINTS "${WICKRA_STRATEGY_CI_LIB_DIR}"
    REQUIRED)

add_executable(run run.c)
target_include_directories(run PRIVATE "${WICKRA_STRATEGY_CI_INCLUDE}")
target_link_libraries(run PRIVATE "${WICKRA_STRATEGY_CI_LIB}")

add_executable(runpp run.cpp)
target_include_directories(runpp PRIVATE "${WICKRA_STRATEGY_CI_INCLUDE}")
target_link_libraries(runpp PRIVATE "${WICKRA_STRATEGY_CI_LIB}")

# The cross-language golden, driven through a real C compiler. Every other
# binding runs golden/ through its own client; without this the hub that the
# C++, C#, Go, Java and R bindings all call through was only ever exercised from
# Rust. The corpus path is compiled in, so the test does not guess where the
# repository root is.
add_executable(golden golden_test.c)
target_include_directories(golden PRIVATE "${WICKRA_STRATEGY_CI_INCLUDE}")
target_link_libraries(golden PRIVATE "${WICKRA_STRATEGY_CI_LIB}")
target_compile_definitions(golden PRIVATE
    WKSTRATEGYCI_GOLDEN_DIR="${CMAKE_CURRENT_SOURCE_DIR}/../../golden")

# On Windows the runtime DLL must sit next to the executable (there is no rpath);
# without it the loader pops a modal "DLL not found" dialog that hangs ctest. Copy
# it next to each example after build. (Unix resolves the .so/.dylib via the
# linker's embedded path, so this is Windows-only.)
if(WIN32)
    foreach(example run runpp golden)
        add_custom_command(TARGET ${example} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${WICKRA_STRATEGY_CI_LIB_DIR}/wickra_strategy_ci.dll"
                "$<TARGET_FILE_DIR:${example}>"
            VERBATIM)
    endforeach()
endif()

enable_testing()
add_test(NAME run COMMAND run)
add_test(NAME runpp COMMAND runpp)
add_test(NAME golden COMMAND golden)
# A missing runtime dependency must fail fast, never block the runner for hours.
set_tests_properties(run runpp golden PROPERTIES TIMEOUT 60)
