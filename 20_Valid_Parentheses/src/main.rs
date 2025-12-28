use valid_parentheses::Solution;

fn main() {
    let test_cases = vec!["()", "()[]{}", "(]", "([)]", "{[]}"];

    for s in test_cases {
        println!("Input: \"{}\" -> {}", s, Solution::is_valid(s.to_string()));
    }
}
