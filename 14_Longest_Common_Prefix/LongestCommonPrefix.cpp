#include <iostream>
#include <string>
#include <vector>

using std::string;
using std::vector;

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        string result = "";

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

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}