use std::cell::RefCell;
use std::rc::Rc;

use crate::p33::TreeNode;
use crate::solution::Solution;

impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree(nums: &[i32], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if index >= nums.len() || nums[index] == 0 {
                return None;
            }

            let node = Rc::new(RefCell::new(TreeNode::new(nums[index])));
            node.borrow_mut().left = build_tree(nums, 2 * index + 1);
            node.borrow_mut().right = build_tree(nums, 2 * index + 2);
            Some(node)
        }

        build_tree(&nums, 0)
    }

    #[test]
    fn test_example1() {
        let root = vec_to_tree(vec![1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1]);
        assert_eq!(Solution::longest_zig_zag(root), 3);
    }

    #[test]
    fn test_example2() {
        let root = vec_to_tree(vec![1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1]);
        assert_eq!(Solution::longest_zig_zag(root), 4);
    }
}
