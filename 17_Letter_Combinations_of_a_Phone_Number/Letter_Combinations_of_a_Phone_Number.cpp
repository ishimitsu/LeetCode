#include <iostream>
#include <vector>
#include <string>

using std::vector;
using std::string;

class Solution {
      const std::string digitToLetters[10] = {
      "",     // 0
      "",     // 1
      "abc",  // 2
      "def",  // 3
      "ghi",  // 4
      "jkl",  // 5
      "mno",  // 6
      "pqrs", // 7
      "tuv",  // 8
      "wxyz"  // 9
  };
public:
    // Given a string containing digits from 2-9 inclusive,
    // return all possible letter combinations that the number could represent.
    //
    // Mapping (like on telephone buttons):
    // 2 -> abc, 3 -> def, 4 -> ghi, 5 -> jkl,
    // 6 -> mno, 7 -> pqrs, 8 -> tuv, 9 -> wxyz
    vector<string> letterCombinations(string digits) {
        vector<string> result = {};
        vector<string> letters = {};
        for (char cdigit : digits) {
            int digit = cdigit - '0'; // This code needs char_digit is "0" - "9"
            if (digit > 9) return {}; // invalid digit!
            letters.push_back(digitToLetters[digit]);
        }

        std::cout << "letters" << std::endl;
        for(string s : letters) { std::cout << s << std::endl; }

        return result;
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
