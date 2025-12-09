cmake_minimum_required(VERSION 3.14)
project(3Sum)

set(CMAKE_CXX_STANDARD 17)

# GoogleTest
find_package(GTest REQUIRED)

# Test executable
add_executable(test_3Sum 3Sum.cpp)
target_link_libraries(test_3Sum GTest::gtest GTest::gtest_main)
