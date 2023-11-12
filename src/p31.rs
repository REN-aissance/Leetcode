use crate::p29::ListNode;
use crate::solution::Solution;

/// Not a good challenge for rust. No idea why using Option<Box<ListNode>>
/// They should have used LinkedList

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list_empty_list() {
        let input = None;
        let expected = None;
        assert_eq!(Solution::reverse_list(input), expected);
    }

    #[test]
    fn test_reverse_list_two_nodes() {
        let input = ListNode::from(vec![1, 2]);
        let expected = ListNode::from(vec![2, 1]);
        assert_eq!(Solution::reverse_list(input), expected);
    }

    #[test]
    fn test_reverse_list_multiple_nodes() {
        let input = ListNode::from(vec![1, 2, 3, 4, 5]);
        let expected = ListNode::from(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(input), expected);
    }
}
