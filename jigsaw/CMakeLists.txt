cmake_minimum_required(VERSION 3.22.1)
project(jigsaw LANGUAGES CXX C)

add_library(jigsaw jigsaw.cpp)

add_executable(app app.cpp)

target_link_libraries(app jigsaw)

add_library(rusty_jigsaw ../../src/bindings.cpp)

target_link_libraries(rusty_jigsaw jigsaw)

add_executable(capp c_app.c)

target_link_libraries(capp rusty_jigsaw)
