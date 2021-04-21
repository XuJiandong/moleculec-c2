cmake_minimum_required(VERSION 3.17)
project(moleculec-c2-types C)

set(CMAKE_C_STANDARD 11)

# uncomment it for sanitize
#set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fsanitize=address -fsanitize=undefined")

# uncomment it for coverage test
# should work with "debug" version (-O0 -g)
#set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} --coverage")

include_directories(../../include)

add_executable(types types.c types-builder.c types-api2.h
        ../../include/molecule_builder.h
        ../../include/molecule_reader.h
        ../../include/molecule2_reader.h)
