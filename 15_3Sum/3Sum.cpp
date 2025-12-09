#include <iostream>
#include <vector>

using std::vector;

class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        vector<vector<int>> result = {};
        std::sort(nums.begin(), nums.end());

        for(int fst = 0; fst < nums.size(); fst++)  {
            if(fst > 0 && nums[fst] == nums[fst - 1]) continue; // fst is duplicated, skip

            int left = fst + 1;
            int right = nums.size() - 1;
            while (left < right) {
                int threeSum = nums[fst] + nums[left] + nums[right];

                if (threeSum > 0) { right--; }
                else if (threeSum < 0) { left++; }
                else { // threeSum == 0
                    result.push_back({nums[fst], nums[left], nums[right]});

                    // skip duplicated left/right
                    while(left < right && nums[left] == nums[left + 1]) left++;
                    while(left < right && nums[right] == nums[right - 1]) right--;
                    left++;
                    right--;
                }
            }
        }

        return result;
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

TEST_F(ThreeSumTest, Example4) {
    vector<int> nums = {2,-3,0,-2,-5,-5,-4,1,2,-2,2,0,2,-4,5,5,-10};
    vector<vector<int>> expected = {{-10,5,5},{-5,0,5},{-4,2,2},{-3,-2,5},{-3,1,2},{-2,0,2}};
    EXPECT_EQ(solution.threeSum(nums), expected);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
