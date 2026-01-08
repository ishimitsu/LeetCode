pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // TODO: implement
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: n = 3
        // Output: ["((()))","(()())","(())()","()(())","()()()"]
        let mut result = Solution::generate_parenthesis(3);
        let mut expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        // Input: n = 1
        // Output: ["()"]
        let result = Solution::generate_parenthesis(1);
        assert_eq!(result, vec!["()".to_string()]);
    }
}
