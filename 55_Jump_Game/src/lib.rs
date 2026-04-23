pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // TODO: implement
        let mut cur_idx = 0;
        let last_idx = nums.len().try_into().unwrap();
        for jmp_num in nums {
            if jmp_num <= 0 { return false }
            cur_idx = cur_idx + jmp_num;
        }

        cur_idx >= last_idx
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
