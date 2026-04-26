pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // TODO: implement
        if nums.len() == 1 { return 0 }
        let last_idx = nums.len()-1;
        let mut jmp_times = 0;
        let mut cur_jmp_reach_max: usize = 0;
        let mut next_jmp_reach_max = 0;

        // implemented with Greedy Algorithm
        for idx in 0..last_idx {
            //if idx > reach_max { return -1 } // It's guaranteed that you can reach last_idx

            let jmp_idx: usize = idx + nums[idx] as usize;  //nums are always >=0, so convert i32 => usize
            if jmp_idx > next_jmp_reach_max {
                // update reach_max of next-jmp
                next_jmp_reach_max = jmp_idx;
            }

            if idx == cur_jmp_reach_max {
                // if idx is max of current-jmp, go next idx with new-jmp.
                jmp_times = jmp_times + 1;
                cur_jmp_reach_max = next_jmp_reach_max;
            }
        }

        jmp_times
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::jump(vec![1,2]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::jump(vec![1,2,1,1,1]), 3);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::jump(vec![7,0,9,6,9,6,1,7,9,0,1,2,9,0,3]), 2);
    }

}
