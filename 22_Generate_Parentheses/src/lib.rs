pub struct Solution;

impl Solution {
    pub fn get_all_parenthesis(parenthesis: &mut String, n: i32, left: i32, right: i32, result: &mut Vec<String>) {

        if left < n {
            parenthesis.push('(');
            Self::get_all_parenthesis (parenthesis, n, left + 1, right, result);
            parenthesis.pop(); // remove ( to pass original parenthesis to under.
        }

        if right < left {
            parenthesis.push(')');
            Self::get_all_parenthesis (parenthesis, n, left, right + 1, result);
            parenthesis.pop(); // remove ( to pass original parenthesis to under.
        }

        if right == n {
            result.push (parenthesis.clone());
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut parenthesis: Vec<String> = Vec::new();
        let mut dummy: String = "".to_string();
        Self::get_all_parenthesis(&mut dummy, n, 0, 0, &mut parenthesis);
        parenthesis
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
