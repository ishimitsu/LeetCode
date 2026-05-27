pub struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        // TODO: implement

        if arr.len() <= 1 { return 0 }
        let mut mov_cnt = 0;
        let mut cur_idx = 0;
        let arr_len = arr.len();
        let mut visited_idx = vec![false; arr.len()];
        let mut queue = vec![];
        queue.push( (cur_idx, mov_cnt) );
        visited_idx[cur_idx] = true;

        // BFS algorithm
        while !queue.is_empty() {
            (cur_idx, mov_cnt) = queue.pop().unwrap();
            mov_cnt = mov_cnt + 1;

            if cur_idx + 1 < arr_len {
                let forward_idx = cur_idx + 1;
                if visited_idx[forward_idx] == false {
                    if forward_idx == arr_len - 1 { return mov_cnt}
                    visited_idx[forward_idx] = true;
                    queue.push( (forward_idx, mov_cnt) );
                }
            }

            if cur_idx >= 1 {
                let back_idx: usize = cur_idx - 1;
                if visited_idx[back_idx] == false {
                    if back_idx == arr_len - 1 { return mov_cnt }
                    visited_idx[back_idx] = true;
                    queue.push( (back_idx, mov_cnt) );
                }
            }

            for same_val_idx in 0..arr_len {
                if visited_idx[same_val_idx] == false
                && same_val_idx != cur_idx
                && arr[same_val_idx] == arr[cur_idx] {
                    if same_val_idx == arr_len - 1 { return mov_cnt }
                    visited_idx[same_val_idx] = true;
                    queue.push( (same_val_idx, mov_cnt) );
                }
            }
        }

        -1
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
}
