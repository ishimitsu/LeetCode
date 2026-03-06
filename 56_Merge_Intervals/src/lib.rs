use std::vec;

pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // TODO: implement
        let mut sorted = intervals;
        if sorted.len() == 0 { return sorted; }
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
    fn test1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(
            Solution::merge(intervals),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn test2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        assert_eq!(Solution::merge(intervals), vec![vec![1, 5]]);
    }

    #[test]
    fn test3() {
        let intervals = vec![vec![4, 7], vec![1, 4]];
        assert_eq!(Solution::merge(intervals), vec![vec![1, 7]]);
    }
}
