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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut dummy;

        while prev.next.is_some() && prev.next.as_ref().unwrap().next.is_some() {
            let mut first = prev.next.take().unwrap();
            let mut second = first.next.take().unwrap();

            first.next = second.next.take();
            second.next = Some(first);
            prev.next = Some(second);

            // prev goes to first->second->next
            prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        dummy.next
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
        // Input: head = [1,2,3,4]
        // Output: [2,1,4,3]
        let head = vec_to_list(vec![1, 2, 3, 4]);
        let result = Solution::swap_pairs(head);
        assert_eq!(list_to_vec(result), vec![2, 1, 4, 3]);
    }

    #[test]
    fn example2() {
        // Input: head = []
        // Output: []
        let head = vec_to_list(vec![]);
        let result = Solution::swap_pairs(head);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn example3() {
        // Input: head = [1]
        // Output: [1]
        let head = vec_to_list(vec![1]);
        let result = Solution::swap_pairs(head);
        assert_eq!(list_to_vec(result), vec![1]);
    }
}
