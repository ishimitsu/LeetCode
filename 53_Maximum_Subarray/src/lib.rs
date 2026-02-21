pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // TODO: implement
        if nums.len() < 0 { return 0;}
        else if nums.len() == 1 { return nums[0]; }

        let mut max_sum = nums[0];
        for left in 0..nums.len() {
            for right in (left+1)..(nums.len()+1) {
                let sum = nums[left..right].iter().sum();
                println!("left:{}, right:{}, cur:{}, max:{}", left, right, sum, max_sum);
                if sum > max_sum {
                    max_sum = sum;
                }
            }
        }

        max_sum
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
