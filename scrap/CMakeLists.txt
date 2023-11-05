cmake_minimum_required(VERSION 3.10)

project(scrap)
add_executable(scrap 
	src/main.cpp)

target_link_libraries(scrap pthread zmq)
