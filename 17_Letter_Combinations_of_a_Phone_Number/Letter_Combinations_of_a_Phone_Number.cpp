#include <iostream>
#include <vector>
#include <string>

using std::vector;
using std::string;

class Solution {
public:
    // Given a string containing digits from 2-9 inclusive,
    // return all possible letter combinations that the number could represent.
    //
    // Mapping (like on telephone buttons):
    // 2 -> abc, 3 -> def, 4 -> ghi, 5 -> jkl,
    // 6 -> mno, 7 -> pqrs, 8 -> tuv, 9 -> wxyz
    vector<string> letterCombinations(string digits) {
        // TODO: implement
        return {};
    }
};

#include <gtest/gtest.h>
#include <algorithm>

class LetterCombinationsTest : public ::testing::Test {
protected:
    Solution solution;
};

// Example 1: digits = "23"
// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
TEST_F(LetterCombinationsTest, Example1) {
    string digits = "23";
    vector<string> expected = {"ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"};
    vector<string> result = solution.letterCombinations(digits);
    std::sort(result.begin(), result.end());
    std::sort(expected.begin(), expected.end());
    EXPECT_EQ(result, expected);
}

// Example 2: digits = ""
// Output: []
TEST_F(LetterCombinationsTest, Example2) {
    string digits = "";
    vector<string> expected = {};
    EXPECT_EQ(solution.letterCombinations(digits), expected);
}

// Example 3: digits = "2"
// Output: ["a","b","c"]
TEST_F(LetterCombinationsTest, Example3) {
    string digits = "2";
    vector<string> expected = {"a", "b", "c"};
    vector<string> result = solution.letterCombinations(digits);
    std::sort(result.begin(), result.end());
    std::sort(expected.begin(), expected.end());
    EXPECT_EQ(result, expected);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
