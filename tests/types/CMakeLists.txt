cmake_minimum_required(VERSION 3.17)
project(types C)

set(CMAKE_C_STANDARD 11)
include_directories(../../include)

add_executable(types types.c types-builder.c types-api2.h
        ../../include/molecule_builder.h
        ../../include/molecule_reader.h
        ../../include/molecule2_reader.h)
