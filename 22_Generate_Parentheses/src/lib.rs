pub struct Solution;

impl Solution {
    pub fn get_all_parenthesis (parenthesis: String, n: i32, left: i32, right: i32, result: &mut Vec<String>) {

        if left < n {
            Self::get_all_parenthesis (parenthesis.clone() + "(", n, left + 1, right, result);
        }

        if right < left && right < n {
            Self::get_all_parenthesis (parenthesis.clone() + ")", n, left, right + 1, result);
        }

        if right == n {
            result.push (parenthesis);
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // TODO: implement
        let mut parenthesis: Vec<String> = Vec::new();
        Self::get_all_parenthesis("".to_string(), n, 0, 0, &mut parenthesis);
        //println!("{:?}", parenthesis);
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
