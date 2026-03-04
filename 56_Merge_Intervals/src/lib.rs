pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // TODO: implement
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
        // Output: [[1,6],[8,10],[15,18]]
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(
            Solution::merge(intervals),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn example2() {
        // Input: intervals = [[1,4],[4,5]]
        // Output: [[1,5]]
        let intervals = vec![vec![1, 4], vec![4, 5]];
        assert_eq!(Solution::merge(intervals), vec![vec![1, 5]]);
    }
}
