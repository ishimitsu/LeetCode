#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>

using std::vector;

class Solution {
public:
    // Given an integer array nums of length n and an integer target,
    // find three integers in nums such that the sum is closest to target.
    // Return the sum of the three integers.
    // You may assume that each input would have exactly one solution.
    int threeSumClosest(vector<int>& nums, int target) {
        // TODO: implement
        return 0;
    }
};

#include <gtest/gtest.h>

class ThreeSumClosestTest : public ::testing::Test {
protected:
    Solution solution;
};

// Example 1: nums = [-1,2,1,-4], target = 1
// Output: 2
// Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
TEST_F(ThreeSumClosestTest, Example1) {
    vector<int> nums = {-1, 2, 1, -4};
    int target = 1;
    EXPECT_EQ(solution.threeSumClosest(nums, target), 2);
}

// Example 2: nums = [0,0,0], target = 1
// Output: 0
// Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
TEST_F(ThreeSumClosestTest, Example2) {
    vector<int> nums = {0, 0, 0};
    int target = 1;
    EXPECT_EQ(solution.threeSumClosest(nums, target), 0);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
