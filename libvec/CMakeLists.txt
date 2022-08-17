# File: libvec/CMakeLists.txt
cmake_minimum_required(VERSION 3.0)
project(libvec C)

add_library(vec STATIC vec.c)

install(TARGETS vec DESTINATION .)