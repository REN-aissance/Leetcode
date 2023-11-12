use crate::solution::Solution;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        let mut stack = vec![(root, 0)];
        while let Some((node, mut depth)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                depth += 1;
                stack.push((node.left.clone(), depth));
                stack.push((node.right.clone(), depth));
                max = std::cmp::max(max, depth);
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth_example1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        let expected = 3;
        assert_eq!(Solution::max_depth(root), expected);
    }

    #[test]
    fn test_max_depth_example2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        let expected = 2;
        assert_eq!(Solution::max_depth(root), expected);
    }
}
