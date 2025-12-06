#include <iostream>
#include <string>
#include <vector>

using std::string;
using std::vector;

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if (strs.empty()) return "";
        if (strs.size() == 1) return strs[0];

        string result = "";
        for (int i = 0; i < strs[0].length(); i++) {
            char searchChar = strs[0][i];
            // Vertical Search
            for (int j = 1; j < strs.size(); j++) {
                if (searchChar != strs[j][i]) {
                    return result;
                }
            }

            result += searchChar;
        }

        return result;
    }
};

#include <gtest/gtest.h>

class LongestCommonPrefixTest : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(LongestCommonPrefixTest, Example1) {
    vector<string> strs = {"flower", "flow", "flight"};
    EXPECT_EQ(solution.longestCommonPrefix(strs), "fl");
}

TEST_F(LongestCommonPrefixTest, Example2) {
    vector<string> strs = {"dog", "racecar", "car"};
    EXPECT_EQ(solution.longestCommonPrefix(strs), "");
}

TEST_F(LongestCommonPrefixTest, Example3) {
    vector<string> strs = {"flower", "flower", "flower"};
    EXPECT_EQ(solution.longestCommonPrefix(strs), "flower");
}

TEST_F(LongestCommonPrefixTest, Example4) {
    vector<string> strs = {"aca", "cba"};
    EXPECT_EQ(solution.longestCommonPrefix(strs), "");
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}