pub struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        // TODO: implement
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
