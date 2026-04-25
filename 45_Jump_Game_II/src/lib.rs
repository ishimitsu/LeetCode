pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // TODO: implement
        if nums.len() == 1 { return 0 }
        let last_idx = nums.len() - 1;
        let mut reach_max = 0;
        let mut jmp_times = 0;
        let mut min_jmp_times = last_idx as i32;

        // implemented with Greedy Algorithm
        for idx in 0..last_idx {
            //if idx > reach_max { return -1 } // It's guaranteed that you can reach last_idx

            let jmp_idx = idx + nums[idx] as usize;  //nums are always >=0, so convert i32 => usize
            jmp_times = jmp_times + 1;
            if jmp_idx > reach_max {
                // if reach_max is updated, you can jump all 0..reach_max
                // so you only need to check reach_max
                reach_max = jmp_idx;

                if reach_max >= last_idx {
                    if jmp_times < min_jmp_times { min_jmp_times = jmp_times; }
                    println!("{}, {}, {}. {}", jmp_idx, reach_max, jmp_times, min_jmp_times);

                    break;
                }
            }
        }

        min_jmp_times
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
