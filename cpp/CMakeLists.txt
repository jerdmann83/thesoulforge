cmake_minimum_required(VERSION 3.10)

project(conc)
add_executable(conc conc.cpp)

target_link_libraries(conc pthread)

