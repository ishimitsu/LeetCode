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
    pub fn has_k_nodes(node: &ListNode, k: i32) -> bool {
        let mut current = &node.next;
        for _ in 0..k {
            match current {
                Some(n) => current = &n.next,
                None => return false,
            }
        }
        true
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // TODO: implement
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut dummy;

        // while prev.next.is_some() && Self::has_k_nodes(prev, k) {
        while Self::has_k_nodes(prev, k) {

            let mut k_head = prev.next.take().unwrap();
            for _ in 0..k-1 {
                let mut k_next = k_head.next.take().unwrap();
                k_head.next = k_next.next.take();
                k_next.next = prev.next.take();
                prev.next = Some(k_next);
            }

            // prev of k_head is none, so need to connect end of k and k_head
            if k > 1 {
                let mut k_end = prev.next.as_mut().unwrap();
                for _ in 0..k-2 {
                    k_end = k_end.next.as_mut().unwrap();
                }
                k_end.next = Some(k_head);
            } else {
                prev.next = Some(k_head);
            }

            // prev goes to next k node
            for _ in 0..k {
                prev = prev.next.as_mut().unwrap(); // move prev to k
            }

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

    #[test]
    fn example3() {
        // Input: head = [1,2,3,4,5], k = 3
        // Output: [3,2,1,4,5]
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let result = Solution::reverse_k_group(head, 1);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 4, 5]);
    }
}
