cmake_minimum_required (VERSION 2.6)
project (fatptr)
set(CMAKE_CXX_FLAGS
    -std=c99
)

set(CMAKE_BUILD_TYPE Debug)

add_executable(fatptr src/fatptr.c)
#target_link_libraries(uvserv uv pthread)
