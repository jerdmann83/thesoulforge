cmake_minimum_required (VERSION 2.6)
project (uvserv)
set(CMAKE_CXX_FLAGS
    -std=c99
)

add_executable(uvserv main.c)
target_link_libraries(uvserv uv pthread)
