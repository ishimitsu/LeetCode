cmake_minimum_required(VERSION 3.14)
project(RomanToInt)

set(CMAKE_CXX_STANDARD 17)

# GoogleTest
find_package(GTest REQUIRED)

# Test executable
add_executable(test_RomanToInt RomanToInt.cpp)
target_link_libraries(test_RomanToInt GTest::gtest GTest::gtest_main)