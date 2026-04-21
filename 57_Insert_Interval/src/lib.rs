pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // TODO: implement
        let mut sorted = intervals;
         // push new interval into intervals and sort
        sorted.push(new_interval);
        sorted.sort_by_key(|interval| interval[0]);

        let mut result: Vec<Vec<i32>> = vec![];

        for interval in sorted {

            if result.is_empty() || interval[0] > result.last().unwrap()[1] {
                // not duplicate, and interval is bigger than result
                result.push(interval);
            } else if interval[1] > result.last().unwrap()[1] {
                // After sorted, if duplicated interval[1] is smaller than result[1],
                // it is always covered by result
                // so only check interval[1] bigger case, and update result[1]

                result.last_mut().unwrap()[1] = interval[1];
            }
        }

        result
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
