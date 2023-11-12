use crate::p33::TreeNode;
use crate::solution::Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Self::path_sum_r(&root, target_sum, vec![])
    }

    fn path_sum_r(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        mut sums: Vec<i64>,
    ) -> i32 {
        if let Some(ref node) = root {
            let node = node.borrow();
            let mut count = if node.val == target_sum { 1 } else { 0 };
            for sum in sums.iter_mut() {
                *sum += node.val as i64;
                if *sum == target_sum as i64 {
                    count += 1;
                }
            }
            sums.push(node.val as i64);
            count
                + Self::path_sum_r(&node.left, target_sum, sums.clone())
                + Self::path_sum_r(&node.right, target_sum, sums)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![10, 5, -3, 3, 2, 0, 11, 3, -2, 0, 1];
        let root = vec_to_tree(nums);
        let target_sum = 8;
        assert_eq!(Solution::path_sum(root, target_sum), 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![5, 4, 8, 11, 0, 13, 4, 7, 2, 0, 0, 5, 1];
        let root = vec_to_tree(nums);
        let target_sum = 22;
        assert_eq!(Solution::path_sum(root, target_sum), 3);
    }

    #[test]
    fn example_3() {
        let nums = vec![
            1000000000, 1000000000, 0, 294967296, 0, 1000000000, 0, 1000000000, 0, 1000000000,
        ];
        let root = vec_to_tree(nums);
        let target_sum = 0;
        assert_eq!(Solution::path_sum(root, target_sum), 0);
    }

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
}
