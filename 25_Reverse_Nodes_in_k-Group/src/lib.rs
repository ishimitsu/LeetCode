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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // TODO: implement
        head
    }
}

// Helper function to create a linked list from a vector
pub fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

// Helper function to convert a linked list to a vector
pub fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    while let Some(node) = head {
        vec.push(node.val);
        head = node.next;
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Input: head = [1,2,3,4,5], k = 2
        // Output: [2,1,4,3,5]
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let result = Solution::reverse_k_group(head, 2);
        assert_eq!(list_to_vec(result), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn example2() {
        // Input: head = [1,2,3,4,5], k = 3
        // Output: [3,2,1,4,5]
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let result = Solution::reverse_k_group(head, 3);
        assert_eq!(list_to_vec(result), vec![3, 2, 1, 4, 5]);
    }
}
