class Solution {
public:
    bool isPalindrome(int x) {
        return true;
    }
};

#include <gtest/gtest.h>

class PalindromeTest : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(PalindromeTest, PositivePalindrome) {
    EXPECT_TRUE(solution.isPalindrome(121));
}

TEST_F(PalindromeTest, NegativeNumbers) {
    EXPECT_FALSE(solution.isPalindrome(-121));
}

TEST_F(PalindromeTest, TrailingZeros) {
    EXPECT_FALSE(solution.isPalindrome(10));
}

TEST_F(PalindromeTest, SingleDigit) {
    EXPECT_TRUE(solution.isPalindrome(0));
    EXPECT_TRUE(solution.isPalindrome(7));
    EXPECT_TRUE(solution.isPalindrome(9));
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}

