cmake_minimum_required(VERSION 3.14)
project(ValidParentheses)

set(CMAKE_CXX_STANDARD 17)

# GoogleTest
find_package(GTest REQUIRED)

# Test executable
add_executable(test_ValidParentheses ValidParentheses.cpp)
target_link_libraries(test_ValidParentheses GTest::gtest GTest::gtest_main)
