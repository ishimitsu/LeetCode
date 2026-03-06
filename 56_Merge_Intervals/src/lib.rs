pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // TODO: implement
        let mut result = intervals;
        if result.len() == 0 { return result; }

        println!("{:?}", result);

        result.sort_by_key(|interval| interval[0]);

        println!("{:?}", result);

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
