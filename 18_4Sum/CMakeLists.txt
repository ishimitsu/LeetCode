cmake_minimum_required(VERSION 3.14)
project(4Sum)

set(CMAKE_CXX_STANDARD 17)

# GoogleTest
find_package(GTest REQUIRED)

# Test executable
add_executable(test_4Sum 4Sum.cpp)
target_link_libraries(test_4Sum GTest::gtest GTest::gtest_main)
