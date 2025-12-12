cmake_minimum_required(VERSION 3.14)
project(Letter_Combinations_of_a_Phone_Number)

set(CMAKE_CXX_STANDARD 17)

# GoogleTest
find_package(GTest REQUIRED)

# Test executable
add_executable(test_Letter_Combinations_of_a_Phone_Number Letter_Combinations_of_a_Phone_Number.cpp)
target_link_libraries(test_Letter_Combinations_of_a_Phone_Number GTest::gtest GTest::gtest_main)
