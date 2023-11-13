use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::p33::TreeNode;
use crate::solution::Solution;

impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        let mut stack = vec![(root.unwrap(), true, 0)];
        while let Some((node, left, cur)) = stack.pop() {
            let node = node.borrow();
            max = max.max(cur);
            if let Some(left_child) = node.left.clone() {
                stack.push((left_child.clone(), true, if left { 1 } else { cur + 1 }));
            }
            if let Some(right_child) = node.right.clone() {
                stack.push((right_child.clone(), false, if left { cur + 1 } else { 1 }));
            }
        }
        max
    }

    /// I struggled a lot on making this function
    /// Learned that accidentally importing std::borrow::BorrowMut
    /// Will break the Rc<RefCell> borrow_mut behavior.
    /// The error message for this makes almost no sense.
    /// Also what in the world is this array format for trees?
    pub fn vec_to_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
        let mut q = VecDeque::new();
        q.push_back(root.clone());

        for chunk in nums[1..].chunks(2) {
            let cur = q.pop_front().unwrap();
            let (l, r) = match chunk {
                [left, right] => (*left, *right),
                _ => return None,
            };
            if l == 1 {
                let left_node = Rc::new(RefCell::new(TreeNode::new(l)));
                cur.borrow_mut().left = Some(left_node.clone());
                q.push_back(left_node);
            }
            if r == 1 {
                let right_node = Rc::new(RefCell::new(TreeNode::new(r)));
                cur.borrow_mut().right = Some(right_node.clone());
                q.push_back(right_node);
            }
        }
        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let root = Solution::vec_to_tree(vec![1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1]);
        assert_eq!(Solution::longest_zig_zag(root), 3);
    }

    #[test]
    fn test_example2() {
        let root = Solution::vec_to_tree(vec![1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1]);
        assert_eq!(Solution::longest_zig_zag(root), 4);
    }

    #[test]
    fn test_simplified_case() {
        let root = Solution::vec_to_tree(vec![1, 1, 1]);
        assert_eq!(Solution::longest_zig_zag(root), 1);
    }
}
