#include <algorithm>
#include <vector>

using std::sort;
using std::vector;

class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        // TODO: implement
        return {};
    }
};

#include <gtest/gtest.h>

class FourSumTest : public ::testing::Test {
protected:
    Solution solution;
};

// Example 1: nums = [1,0,-1,0,-2,2], target = 0
// Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
TEST_F(FourSumTest, Example1) {
    vector<int> nums = {1, 0, -1, 0, -2, 2};
    int target = 0;
    vector<vector<int>> expected = {{-2, -1, 1, 2}, {-2, 0, 0, 2}, {-1, 0, 0, 1}};
    vector<vector<int>> result = solution.fourSum(nums, target);
    sort(result.begin(), result.end());
    sort(expected.begin(), expected.end());
    EXPECT_EQ(result, expected);
}

// Example 2: nums = [2,2,2,2,2], target = 8
// Output: [[2,2,2,2]]
TEST_F(FourSumTest, Example2) {
    vector<int> nums = {2, 2, 2, 2, 2};
    int target = 8;
    vector<vector<int>> expected = {{2, 2, 2, 2}};
    vector<vector<int>> result = solution.fourSum(nums, target);
    EXPECT_EQ(result, expected);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
