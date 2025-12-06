cmake_minimum_required(VERSION 3.14)
project(LongestCommonPrefix)

set(CMAKE_CXX_STANDARD 17)

# GoogleTest
find_package(GTest REQUIRED)

# Test executable
add_executable(test_LongestCommonPrefix LongestCommonPrefix.cpp)
target_link_libraries(test_LongestCommonPrefix GTest::gtest GTest::gtest_main)