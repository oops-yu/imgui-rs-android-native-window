cmake_minimum_required(VERSION 3.22.1)
project(native-window LANGUAGES CXX C)
set(CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -flto -Os")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -flto -Os")
include_directories(
    include
)

file(GLOB SOURCE_FILES  src/*.c* src/*/*.c*) 


add_library(${PROJECT_NAME} STATIC ${SOURCE_FILES})

