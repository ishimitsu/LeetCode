#include <iostream>
#include <string>
#include <stack>

using std::string;
using std::stack;

class Solution {
public:
    bool isValid(string s) {
        // TODO: implement
        return false;
    }
};

#include <gtest/gtest.h>

class ValidParenthesesTest : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(ValidParenthesesTest, Example1) {
    // Input: s = "()"
    // Output: true
    EXPECT_TRUE(solution.isValid("()"));
}

TEST_F(ValidParenthesesTest, Example2) {
    // Input: s = "()[]{}"
    // Output: true
    EXPECT_TRUE(solution.isValid("()[]{}"));
}

TEST_F(ValidParenthesesTest, Example3) {
    // Input: s = "(]"
    // Output: false
    EXPECT_FALSE(solution.isValid("(]"));
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
