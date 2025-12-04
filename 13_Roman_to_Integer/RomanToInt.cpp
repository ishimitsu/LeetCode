#include <stdio.h>
#include <string>

using std::string;

class Solution {
    enum SymbolValue {
        I = 1,
        V = 5,
        X = 10,
        L = 50,
        C = 100,
        D = 500,
        M = 1000
    };

public:
    int romanToInt(string s) {
        return 0;
    }
};

#include <gtest/gtest.h>

class RomanToIntTest : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(RomanToIntTest, PositiveRomanToIntTest) {
    EXPECT_EQ(solution.romanToInt("III"), 3);
    EXPECT_EQ(solution.romanToInt("IV"), 4);
    EXPECT_EQ(solution.romanToInt("IX"), 9);
    EXPECT_EQ(solution.romanToInt("LVIII"), 58);
    EXPECT_EQ(solution.romanToInt("MCMXCIV"), 1994);
}
