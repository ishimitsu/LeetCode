pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // TODO: implement
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
        // Output: [[1,5],[6,9]]
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        assert_eq!(
            Solution::insert(intervals, new_interval),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn example2() {
        // Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
        // Output: [[1,2],[3,10],[12,16]]
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        assert_eq!(
            Solution::insert(intervals, new_interval),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
