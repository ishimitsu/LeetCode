cmake_minimum_required(VERSION 3.14)
project(PalindromeNumber)

set(CMAKE_CXX_STANDARD 17)

# GoogleTest
find_package(GTest REQUIRED)
include_directories(${GTEST_INCLUDE_DIRS})

# Test executable
add_executable(test_palindrome 9_Palindrome_Number.cpp)
target_link_libraries(test_palindrome ${GTEST_LIBRARIES} pthread)