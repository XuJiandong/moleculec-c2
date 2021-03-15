cmake_minimum_required(VERSION 3.15)
project(moleculec_c2_fuzzer C)

set(CMAKE_C_STANDARD 11)
include_directories(../../include)
include_directories(../blockchain)
add_definitions(-DNO_FUZZER)

add_executable(moleculec_c2_fuzzer moleculec_c2_fuzzer.c ../blockchain/blockchain-builder.h ../blockchain/blockchain-api2.h
        ../../include/molecule_builder.h
        ../../include/molecule2_reader.h)
