use core::num;
use std::i32;

pub struct Solution;

impl Solution {
    pub fn get_max_sub_array (nums: &Vec<i32>, left: usize, right: usize) -> i32 {
        if left == right { return nums[left]; }

        let mid = (left + right) / 2;
        //println!("left:{}, right:{}, mid:{}", left, right, mid);
        let left_max = Self::get_max_sub_array(&nums, left, mid);
        let right_max = Self::get_max_sub_array(&nums, mid+1, right);

        let mut cross_max_left = i32::MIN;
        let mut cross_max_right = i32::MIN;

        let mut cross_sum_left = 0;
        for l in (left..mid+1).rev() {
            cross_sum_left += nums[l];
            cross_max_left = if cross_sum_left > cross_max_left { cross_sum_left } else { cross_max_left };
        }
        let mut cross_sum_right = 0;
        for r in (mid+1)..(right+1) {
            cross_sum_right += nums[r];
            cross_max_right = if cross_sum_right > cross_max_right { cross_sum_right } else { cross_max_right };
        }
        let cross_max = cross_max_left + cross_max_right;

        let mut max_sum;
        max_sum = if left_max > right_max { left_max } else { right_max };
        max_sum = if cross_max > max_sum { cross_max } else { max_sum };
        //println!("left:{}, right:{}, max:{} {} {} {}", left, right, left_max, right_max, cross_max, max_sum);

        max_sum
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // TODO: implement
        if nums.is_empty() { return 0;}
        else if nums.len() == 1 { return nums[0]; }
        Self::get_max_sub_array(&nums, 0, nums.len()-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
        // Output: 6
        assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn example2() {
        // Input: nums = [1]
        // Output: 1
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn example3() {
        // Input: nums = [5,4,-1,7,8]
        // Output: 23
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);
    }
}
