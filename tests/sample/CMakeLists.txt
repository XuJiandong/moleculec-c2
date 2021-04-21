cmake_minimum_required(VERSION 3.17)
project(moleculec-c2-sample C)

set(CMAKE_C_STANDARD 11)
include_directories(../../include)

add_executable(sample sample.c sample-builder.h sample-api2.h
        ../../include/molecule_builder.h
        ../../include/molecule_reader.h
        ../../include/molecule2_reader.h)
