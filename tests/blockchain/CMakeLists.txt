cmake_minimum_required(VERSION 3.15)
project(blockchain C)

set(CMAKE_C_STANDARD 11)
include_directories(../../include)

add_executable(blockchain blockchain.c blockchain-builder.h blockchain-api2.h
        ../../include/molecule_builder.h
        ../../include/molecule_reader.h
        ../../include/molecule2_reader.h)
