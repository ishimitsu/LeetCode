pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 { return true }
        let last_idx = nums.len() - 1;
        let mut cur_reach_max = 0;

        // implemented with Greedy Algorithm
        for idx in 0..last_idx {
            if idx > cur_reach_max { return false } // can't reach max

            let jmp_idx = idx + nums[idx] as usize;  // elements in nums always 0>=0
            if jmp_idx > cur_reach_max {
                // if reach_max is updated, you can jump all 0..reach_max
                // so you only need to check reach_max
                cur_reach_max = jmp_idx;
                if cur_reach_max >= last_idx { break }
            }
        }

        cur_reach_max >= last_idx
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

    #[test]
    fn example3() {
        // Output: true
        assert_eq!(Solution::can_jump(vec![1]), true);
    }

        #[test]
    fn example4() {
        // Output: true
        assert_eq!(Solution::can_jump(vec![2, 5, 0, 0]), true);
    }
}
