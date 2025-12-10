cmake_minimum_required(VERSION 3.14)
project(3Sum_Closest)

set(CMAKE_CXX_STANDARD 17)

# GoogleTest
find_package(GTest REQUIRED)

# Test executable
add_executable(test_3Sum_Closest 3Sum_Closest.cpp)
target_link_libraries(test_3Sum_Closest GTest::gtest GTest::gtest_main)
