fn try_visit(
    next_idx: usize,
    mov_cnt: i32,
    arr_len: usize,
    visited_idx: &mut Vec<bool>,
    queue: &mut Vec<(usize, i32)>,
    min_mov_cnt: &mut i32,
) {
    if !visited_idx[next_idx] {
        visited_idx[next_idx] = true;
        queue.push((next_idx, mov_cnt));
        if next_idx == arr_len - 1 && *min_mov_cnt > mov_cnt {
            *min_mov_cnt = mov_cnt;
        }
    }
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
        let mut queue = vec![];
        queue.push((cur_idx, mov_cnt));
        visited_idx[cur_idx] = true;

        // BFS algorithm
        while !queue.is_empty() {
            (cur_idx, mov_cnt) = queue.pop().unwrap();
            mov_cnt = mov_cnt + 1;

            if cur_idx + 1 < arr_len {
                try_visit(cur_idx + 1, mov_cnt, arr_len, &mut visited_idx, &mut queue, &mut min_mov_cnt);
            }

            if cur_idx >= 1 {
                try_visit(cur_idx - 1, mov_cnt, arr_len, &mut visited_idx, &mut queue, &mut min_mov_cnt);
            }

            for same_val_idx in 0..arr_len {
                if same_val_idx != cur_idx && arr[same_val_idx] == arr[cur_idx] {
                    try_visit(same_val_idx, mov_cnt, arr_len, &mut visited_idx, &mut queue, &mut min_mov_cnt);
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
