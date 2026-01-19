// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // TODO: implement
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a linked list from a vector
    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }

    // Helper function to convert a linked list to a vector
    fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }

    #[test]
    fn example1() {
        // Input: lists = [[1,4,5],[1,3,4],[2,6]]
        // Output: [1,1,2,3,4,4,5,6]
        let lists = vec![
            to_list(vec![1, 4, 5]),
            to_list(vec![1, 3, 4]),
            to_list(vec![2, 6]),
        ];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(to_vec(result), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn example2() {
        // Input: lists = []
        // Output: []
        let lists: Vec<Option<Box<ListNode>>> = vec![];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(to_vec(result), vec![]);
    }

    #[test]
    fn example3() {
        // Input: lists = [[]]
        // Output: []
        let lists = vec![to_list(vec![])];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(to_vec(result), vec![]);
    }
}
