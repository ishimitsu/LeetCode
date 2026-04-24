pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // TODO: implement
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: nums = [2,3,1,1,4]
        // Output: 2
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn example2() {
        // Input: nums = [2,3,0,1,4]
        // Output: 2
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
