cmake_minimum_required(VERSION 3.9)

project(func)
set(func_srcs func.c)
add_library(func ${func_srcs})
install(TARGETS func DESTINATION .)
