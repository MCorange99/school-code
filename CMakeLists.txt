cmake_minimum_required(VERSION 3.5)
project(School_code LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)

include_directories(include)

add_subdirectory(calc)
add_subdirectory(test)