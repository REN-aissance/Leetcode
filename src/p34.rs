use crate::solution::Solution;
use std::cell::RefCell;
use std::rc::Rc;
use crate::p33::TreeNode;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let (mut l1, mut l2) = (vec![], vec![]);
        Self::get_leaves(root1, &mut l1);
        Self::get_leaves(root2, &mut l2);
        l1 == l2
    }

    fn get_leaves(node: Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                leaves.push(node.val);
            } else {
                Self::get_leaves(node.left.clone(), leaves);
                Self::get_leaves(node.right.clone(), leaves);
            }
        }
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
    fn test_leaf_similar_example1() {
        let root1 = vec_to_tree(vec![3, 5, 1, 6, 2, 9, 8, 0, 0, 7, 4]);
        let root2 = vec_to_tree(vec![3, 5, 1, 6, 7, 4, 2, 0, 0, 0, 0, 0, 0, 9, 8]);

        assert_eq!(Solution::leaf_similar(root1, root2), true);
    }

    #[test]
    fn test_leaf_similar_example2() {
        let root1 = vec_to_tree(vec![1, 2, 3]);
        let root2 = vec_to_tree(vec![1, 3, 2]);

        assert_eq!(Solution::leaf_similar(root1, root2), false);
    }
}
