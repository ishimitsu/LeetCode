pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' |'{' => v.push(c),
                ')' => if v.pop() != Some('(') { return false }
                ']' => if v.pop() != Some('[') { return false }
                '}' => if v.pop() != Some('{') { return false }
                _ => return false
            }
        }

        true
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
