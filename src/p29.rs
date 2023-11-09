use crate::solution::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;
        let mut n = 1;
        let mut ptr = head.as_ref().unwrap().as_ref();
        while let Some(node) = ptr.next.as_ref() {
            ptr = node.as_ref();
            n += 1;
        }
        if n == 1 {
            return None;
        }
        let mut ptr = head.as_mut().unwrap().as_mut();
        for _ in 0..(n / 2 - 1) {
            ptr = ptr.next.as_mut().unwrap().as_mut();
        }
        ptr.next = ptr.next.as_mut().unwrap().as_mut().next.take();
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = create_linked_list(vec![1, 3, 4, 7, 1, 2, 6]);
        let result = create_linked_list(vec![1, 3, 4, 1, 2, 6]);
        assert_eq!(Solution::delete_middle(input), result);
    }

    #[test]
    fn test_example_2() {
        let input = create_linked_list(vec![1, 2, 3, 4]);
        let result = create_linked_list(vec![1, 2, 4]);
        assert_eq!(Solution::delete_middle(input), result);
    }

    #[test]
    fn test_example_3() {
        let input = create_linked_list(vec![2, 1]);
        let result = create_linked_list(vec![2]);
        assert_eq!(Solution::delete_middle(input), result);
    }

    // Helper function to create a linked list from a vector
    fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}
