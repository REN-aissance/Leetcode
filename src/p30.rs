use crate::p29::ListNode;
use crate::solution::Solution;

/// Not a good challenge for rust. No idea why using Option<Box<ListNode>>
/// They should have used LinkedList

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut odds, mut evens) = (None, None);
        let (mut tail_odds, mut tail_evens) = (&mut odds, &mut evens);
        let mut even = true;

        while let Some(mut node) = head {
            head = node.next;
            node.next = None;
            even = !even; //starts from odd for some reason
            if even {
                tail_evens = &mut tail_evens.insert(node).next;
            } else {
                tail_odds = &mut tail_odds.insert(node).next;
            }
        }

        if let Some(node) = evens {
            let _ = tail_odds.insert(node);
        }

        odds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_even_list_example1() {
        let input = ListNode::from(vec![1, 2, 3, 4, 5]);
        let expected_output = ListNode::from(vec![1, 3, 5, 2, 4]);
        assert_eq!(Solution::odd_even_list(input), expected_output);
    }

    #[test]
    fn test_odd_even_list_example2() {
        let input = ListNode::from(vec![2, 1, 3, 5, 6, 4, 7]);
        let expected_output = ListNode::from(vec![2, 3, 6, 7, 1, 5, 4]);
        assert_eq!(Solution::odd_even_list(input), expected_output);
    }
}
