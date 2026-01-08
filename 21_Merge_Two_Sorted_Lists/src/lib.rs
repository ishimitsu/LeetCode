use std::ptr::null;

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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0); // initialize with dummy val
        let mut tail = &mut head;

        while list1.is_some() || list2.is_some() {
            match (list1.as_ref(), list2.as_ref()) {
                (Some(_), None) => { /* list2 is empty */
                    tail.next = list1;
                    return head.next
                }
                (None, Some(_)) => { /* list1 is empty */
                    tail.next = list2;
                    return head.next
                }
                (None, None) => { return head.next /* both empty */ }
                (Some(n1), Some(n2)) => {
                    if n1.val <= n2.val {
                        tail.next = Some(Box::new(ListNode::new(n1.val)));
                        list1 = list1.unwrap().next;
                    } else {
                        tail.next = Some(Box::new(ListNode::new(n2.val)));
                        list2 = list2.unwrap().next;
                    }
                }
            }
            tail = tail.next.as_mut().unwrap();
        }

        head.next
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
        // Input: list1 = [1,2,4], list2 = [1,3,4]
        // Output: [1,1,2,3,4,4]
        let list1 = to_list(vec![1, 2, 4]);
        let list2 = to_list(vec![1, 3, 4]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn example2() {
        // Input: list1 = [], list2 = []
        // Output: []
        let list1 = to_list(vec![]);
        let list2 = to_list(vec![]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(result), vec![]);
    }

    #[test]
    fn example3() {
        // Input: list1 = [], list2 = [0]
        // Output: [0]
        let list1 = to_list(vec![]);
        let list2 = to_list(vec![0]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(result), vec![0]);
    }
}
