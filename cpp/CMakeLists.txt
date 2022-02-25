cmake_minimum_required(VERSION 3.10)

# set the project name
project(conc)

# add the executable
add_executable(conc conc.cpp)

target_link_libraries(conc pthread)

