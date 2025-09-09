cmake_minimum_required(VERSION 3.10)
project(zmque CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(zmque src/zmque.cpp)
target_link_libraries(zmque zmq)
target_compile_options(zmque PRIVATE -Wall -Wextra)
