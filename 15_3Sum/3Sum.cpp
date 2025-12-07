#include <iostream>
#include <vector>

using std::vector;

class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        // TODO: implement
        return {};
    }
};

#include <gtest/gtest.h>

class ThreeSumTest : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(ThreeSumTest, Example1) {
    vector<int> nums = {-1, 0, 1, 2, -1, -4};
    vector<vector<int>> expected = {{-1, -1, 2}, {-1, 0, 1}};
    EXPECT_EQ(solution.threeSum(nums), expected);
}

TEST_F(ThreeSumTest, Example2) {
    vector<int> nums = {0, 1, 1};
    vector<vector<int>> expected = {};
    EXPECT_EQ(solution.threeSum(nums), expected);
}

TEST_F(ThreeSumTest, Example3) {
    vector<int> nums = {0, 0, 0};
    vector<vector<int>> expected = {{0, 0, 0}};
    EXPECT_EQ(solution.threeSum(nums), expected);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
