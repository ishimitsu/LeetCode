pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // TODO: implement
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: s = "()"
        // Output: true
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn example2() {
        // Input: s = "()[]{}"
        // Output: true
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn example3() {
        // Input: s = "(]"
        // Output: false
        assert!(!Solution::is_valid("(]".to_string()));
    }
}
