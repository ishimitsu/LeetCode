#include <stdio.h>
#include <cmath>

class Solution {
public:
    int getDigitNum(int x) {
        int digitNum = 0;
        int temp = x;
        while(temp / 10 > 0) {
            digitNum++;
            temp /= 10;
        }
        return digitNum;
    }

    bool isPalindrome(int x) {
        if (x < 0) return false;
        if ((x >= 0) && (x <= 9)) return true;

        int digitNum = getDigitNum(x);
        int left = 1;
        int right = std::pow(10, digitNum);

        while (left < right) {
            int leftDigit = (x / left) % 10 ;
            int rightDigit = (x / right) % 10 ;
            printf("leftDigit: %d, rightDigit: %d\n", leftDigit, rightDigit);
            if (leftDigit != rightDigit) {
                return false;
            }

            left = left * 10;
            right = right / 10;
        }

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

