class Solution {
public:
    bool isPalindrome(int x) {
        if (x < 0) return false;
        if (x < 10) return true;
        if (x % 10 == 0) return false; // XX..0 should always false (but 0 is true)

        int reserved = 0;
        int temp = x;

        // cut half-x and reserve, like 1221 => 122/1, 12/21 (stop reserving)
        while (reserved < temp) {
            reserved = reserved * 10 + temp % 10;
            temp /= 10;
        }

        // 12/21 => true, or 12321 => 12/321 (reserved is 123, so remove 3 and compare 12/12)
        return reserved == temp || reserved / 10  == temp;
    }
};

#include <gtest/gtest.h>

class PalindromeTest : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(PalindromeTest, PositivePalindrome) {
    EXPECT_TRUE(solution.isPalindrome(121));
    EXPECT_TRUE(solution.isPalindrome(12321));
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

