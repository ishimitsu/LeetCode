#include <stdio.h>
#include <string>

using std::string;

class Solution {
private:
    static constexpr int getIntValFromRoman(char roman) {
        switch(roman) {
            case 'I': return 1;
            case 'V': return 5;
            case 'X': return 10;
            case 'L': return 50;
            case 'C': return 100;
            case 'D': return 500;
            case 'M': return 1000;
            default: return 0;
        }
    }

    static constexpr int getIntValFrom2Romans(char roman1, char roman2) {
        if (roman1 == 'I' && roman2 == 'V') return 4;
        if (roman1 == 'I' && roman2 == 'X') return 9;
        if (roman1 == 'X' && roman2 == 'L') return 40;
        if (roman1 == 'X' && roman2 == 'C') return 90;
        if (roman1 == 'C' && roman2 == 'D') return 400;
        if (roman1 == 'C' && roman2 == 'M') return 900;
        return 0;
    }

public:
    int romanToInt(string s) {
        int result = 0;
        int idx = 0;

        while (idx + 1 < s.length()) {
            int chk2romans = getIntValFrom2Romans(s[idx], s[idx + 1]);
            if (chk2romans > 0) {
                result += chk2romans;
                idx += 2;
            } else {
                result += getIntValFromRoman(s[idx]);
                idx++;
            }
        }

        // check last roman
        if (idx < s.length()) {
            result += getIntValFromRoman(s[idx]);
            idx++;
        }

        return result;
    }
};

#include <gtest/gtest.h>

class RomanToIntTest : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(RomanToIntTest, TestIII) {
    EXPECT_EQ(solution.romanToInt("III"), 3);
}

TEST_F(RomanToIntTest, TestIV) {
    EXPECT_EQ(solution.romanToInt("IV"), 4);
}

TEST_F(RomanToIntTest, TestIX) {
    EXPECT_EQ(solution.romanToInt("IX"), 9);
}

TEST_F(RomanToIntTest, TestLVIII) {
    EXPECT_EQ(solution.romanToInt("LVIII"), 58);
}

TEST_F(RomanToIntTest, TestMCMXCIV) {
    EXPECT_EQ(solution.romanToInt("MCMXCIV"), 1994);
}