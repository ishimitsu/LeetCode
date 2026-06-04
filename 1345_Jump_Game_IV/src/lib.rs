use std::collections::VecDeque;
use std::collections::HashSet;

fn push_next_idx(
    next_idx: usize,
    mov_cnt: i32,
    visited_idx: &mut Vec<bool>,
    queue: &mut VecDeque<(usize, i32)>,
) {
    if visited_idx[next_idx] == true { return };

    visited_idx[next_idx] = true;
    queue.push_back((next_idx, mov_cnt));
}

pub struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let arr_len = arr.len();
        if arr_len <= 1 { return 0 }

        let mut mov_cnt = 0;
        let mut cur_idx = 0;
        let mut min_mov_cnt = arr_len as i32;
        let mut visited_idx = vec![false; arr.len()];
        let mut checked_val = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((cur_idx, mov_cnt));
        visited_idx[cur_idx] = true;

        // BFS algorithm
        while !queue.is_empty() {
            (cur_idx, mov_cnt) = queue.pop_front().unwrap();

            if cur_idx == arr_len - 1 && min_mov_cnt > mov_cnt {
                min_mov_cnt = mov_cnt;
            } else {
                mov_cnt = mov_cnt + 1;

                if cur_idx + 1 < arr_len {
                    push_next_idx(cur_idx + 1, mov_cnt, &mut visited_idx, &mut queue);
                }

                if cur_idx >= 1 {
                    push_next_idx(cur_idx - 1, mov_cnt, &mut visited_idx, &mut queue);
                }

                let cur_val = arr[cur_idx];
                if checked_val.contains(&cur_val) == false {
                    checked_val.insert(cur_val);

                    for same_val_idx in 0..arr_len {
                        if same_val_idx != cur_idx && arr[same_val_idx] == cur_val {
                            push_next_idx(same_val_idx, mov_cnt, &mut visited_idx, &mut queue);
                        }
                    }
                }
            }

        }

        min_mov_cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
        // Output: 3
        assert_eq!(Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]), 3);
    }

    #[test]
    fn example2() {
        // Input: arr = [7]
        // Output: 0
        assert_eq!(Solution::min_jumps(vec![7]), 0);
    }

    #[test]
    fn example3() {
        // Input: arr = [7,6,9,6,9,6,9,7]
        // Output: 1
        assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::min_jumps(vec![7,7,2,1,7,7,7,3,4,1]), 3);
    }
}
