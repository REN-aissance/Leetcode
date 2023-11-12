
use crate::p29::ListNode;
use crate::solution::Solution;

/// Not a good challenge for rust. No idea why using Option<Box<ListNode>>
/// They should have used LinkedList

impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        // get mid node
        let mut slow = &head;
        let mut fast = &head;
        // as far as it's even length, we can skip fast.next check
        while fast.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        // reverse second half
        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = slow.clone();
        while let Some(mut node) = cur.take() {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            cur = next;
        }
        // traverse from both sides, pick max sum
        let mut back = prev;
        let mut max_sum = i32::MIN;
        while let Some(mut b_node) = back.take() {
            let mut f_node = head.unwrap();
            max_sum = max_sum.max(f_node.val + b_node.val);
            head = f_node.next.take();
            back = b_node.next.take();
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_sum_example1() {
        let input = ListNode::from(vec![5, 4, 2, 1]);
        let expected = 6;
        assert_eq!(Solution::pair_sum(input), expected);
    }

    #[test]
    fn test_pair_sum_example2() {
        let input = ListNode::from(vec![4, 2, 2, 3]);
        let expected = 7;
        assert_eq!(Solution::pair_sum(input), expected);
    }

    #[test]
    fn test_pair_sum_example3() {
        let input = ListNode::from(vec![1, 100000]);
        let expected = 100001;
        assert_eq!(Solution::pair_sum(input), expected);
    }
}
