#include <iostream>
#include <vector>

using std::vector;

class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        vector<vector<int>> result = {};

        for(int fst = 0; fst < nums.size(); fst++)  {
            for(int sec = fst + 1; sec < nums.size(); sec++)  {
                for(int thd = sec + 1; thd < nums.size(); thd++)  {

                    if (nums[fst] + nums[sec] + nums[thd] == 0) {
                        bool duplicated = false;
                        for (vector<int> vec : result) {
                            vector<bool> duplicated_three = {false, false, false};
                            int duplicated_cnt = 0;

                            for (int i = 0; i < vec.size(); i++) {
                                if (!duplicated_three[0] && nums[fst] == vec[i]) {
                                    duplicated_three[0] = true;
                                    duplicated_cnt++;
                                } else if (!duplicated_three[1] && nums[sec] == vec[i]) {
                                    duplicated_three[1] = true;
                                    duplicated_cnt++;
                                } else if (!duplicated_three[2] && nums[thd] == vec[i]) {
                                    duplicated_three[2] = true;
                                    duplicated_cnt++;
                                }

                                if (duplicated_cnt >= 3) {
                                    duplicated = true;
                                    break;
                                }
                            }
                        }

                        if (!duplicated) {
                            vector<int> three_sum = {-1, -1, -1};
                            three_sum[0] = nums[fst];
                            three_sum[1] = nums[sec];
                            three_sum[2] = nums[thd];

                            result.push_back(three_sum);
                        }
                    }
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

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
