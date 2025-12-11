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
        int result = 10001; // -10^4 <= target <= 10^4
        int resultDiff = 20002; // -10^4 ~ 10^4
        std::sort(nums.begin(), nums.end());

        for(int fst = 0; fst < nums.size(); fst++)  {
            if(fst > 0 && nums[fst] == nums[fst - 1]) continue; // fst is duplicated, skip

            int left = fst + 1;
            int right = nums.size() - 1;
            while (left < right) {
                int threeSum = nums[fst] + nums[left] + nums[right];

                if (threeSum == target) return target;

                int threeSumDiff = std::abs(target - threeSum);
                if (threeSumDiff < resultDiff) {
                    resultDiff = threeSumDiff;
                    result = threeSum;

                    // skip duplicated left/right
                    //while(left < right && nums[left] == nums[left + 1]) left++;
                    //while(left < right && nums[right] == nums[right - 1]) right--;
                }

                if (threeSum > target) { right--; }
                else { left++; }
            }
        }

        return result;
    }
};

#include <gtest/gtest.h>

class ThreeSumClosestTest : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(ThreeSumClosestTest, Example1) {
    vector<int> nums = {-1, 2, 1, -4};
    int target = 1;
    EXPECT_EQ(solution.threeSumClosest(nums, target), 2);
}

TEST_F(ThreeSumClosestTest, Example2) {
    vector<int> nums = {0, 0, 0};
    int target = 1;
    EXPECT_EQ(solution.threeSumClosest(nums, target), 0);
}

TEST_F(ThreeSumClosestTest, Example3) {
    vector<int> nums = {4,0,5,-5,3,3,0,-4,-5};
    int target = -2;
    EXPECT_EQ(solution.threeSumClosest(nums, target), -2);
}

TEST_F(ThreeSumClosestTest, Example4) {
    vector<int> nums = {10,20,30,40,50,60,70,80,90};
    int target = 1;
    EXPECT_EQ(solution.threeSumClosest(nums, target), 60);
}

TEST_F(ThreeSumClosestTest, Example5) {
    vector<int> nums = {-1000,-1000,-1000};
    int target = 10000;
    EXPECT_EQ(solution.threeSumClosest(nums, target), -3000);
}


int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
