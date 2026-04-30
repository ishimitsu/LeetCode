pub struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        // TODO: implement
        if arr.len() == 1 {
            if arr[0] == 0 { return true } else { return false }
        }

        let mut cur_idx = start as usize;
        let mut visited_idx = vec![false; arr.len()];
        let mut queue = vec![];
        queue.push(arr[cur_idx]);
        visited_idx[cur_idx] = true;

        // BFS algorithm
        while !queue.is_empty() {
            cur_idx = queue.pop().unwrap() as usize;

            let forward_idx = cur_idx + arr[cur_idx] as usize;
            if forward_idx < arr.len() && visited_idx[forward_idx] == false {
                if arr[forward_idx] == 0 { return true }
                visited_idx[forward_idx] = true;
                queue.push(arr[forward_idx]);
            }

            if cur_idx >= arr[cur_idx] as usize {
                let back_idx = cur_idx - arr[cur_idx] as usize;
                if visited_idx[back_idx] == false {
                    if arr[back_idx] == 0 { return true }
                    visited_idx[back_idx] = true;
                    queue.push(arr[back_idx]);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: arr = [4,2,3,0,3,1,2], start = 5
        // Output: true
        assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
    }

    #[test]
    fn example2() {
        // Input: arr = [4,2,3,0,3,1,2], start = 0
        // Output: true
        assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
    }

    #[test]
    fn example3() {
        // Input: arr = [3,0,2,1,2], start = 2
        // Output: false
        assert_eq!(Solution::can_reach(vec![3, 0, 2, 1, 2], 2), false);
    }
}
