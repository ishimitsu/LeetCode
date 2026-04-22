pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // TODO: implement
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: nums = [2,3,1,1,4]
        // Output: true
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn example2() {
        // Input: nums = [3,2,1,0,4]
        // Output: false
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
