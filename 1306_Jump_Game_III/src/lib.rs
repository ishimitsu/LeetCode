pub struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        // TODO: implement
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
