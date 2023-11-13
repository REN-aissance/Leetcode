use crate::p33::TreeNode;
use crate::solution::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn good_nodes_r(node: Option<Rc<RefCell<TreeNode>>>, mut max: i32) -> i32 {
            let mut count = 0;
            if let Some(node) = node {
                let node = node.borrow();
                if node.val >= max {
                    max = node.val;
                    count += 1;
                }
                count += good_nodes_r(node.left.clone(), max);
                count += good_nodes_r(node.right.clone(), max);
            }
            count
        }

        good_nodes_r(root, i32::MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![3, 1, 4, 3, 0, 1, 5];
        let root = Solution::vec_to_tree(nums);
        assert_eq!(Solution::good_nodes(root), 4);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 3, 0, 4, 2];
        let root = Solution::vec_to_tree(nums);
        assert_eq!(Solution::good_nodes(root), 3);
    }

    #[test]
    fn example_3() {
        let nums = vec![1];
        let root = Solution::vec_to_tree(nums);
        assert_eq!(Solution::good_nodes(root), 1);
    }
}
