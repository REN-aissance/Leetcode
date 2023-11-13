use crate::{p33::TreeNode, solution::Solution};
use std::{cell::RefCell, rc::Rc};
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: OptNode, p: OptNode, q: OptNode) -> OptNode {
        Solution::f(&root, &p.as_ref()?.borrow().val, &q.as_ref()?.borrow().val)
    }

    fn f(root: &OptNode, p: &i32, q: &i32) -> OptNode {
        match root {
            Some(node) => {
                let temp = node.as_ref().borrow();
                if &temp.val == p || &temp.val == q {
                    root.clone()
                } else {
                    let left = Solution::f(&temp.left, p, q);
                    let right = Solution::f(&temp.right, p, q);
                    if left.is_some() && right.is_some() {
                        root.clone()
                    } else {
                        left.or(right)
                    }
                }
            }
            None => None,
        }
    }

    pub fn find_node_value(root: &OptNode, num: i32) -> OptNode {
        if let Some(root) = root.clone() {
            if root.borrow().val == num {
                Some(root)
            } else if let Some(n) = Self::find_node_value(&root.borrow().left, num) {
                Some(n)
            } else {
                Self::find_node_value(&root.borrow().right, num)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        // Input
        let nums = vec![3, 5, 1, 6, 2, 0, 8, 0, 0, 7, 4];
        let root = Solution::vec_to_tree(nums);
        let p = Solution::find_node_value(&root, 5);
        let q = Solution::find_node_value(&root, 1);

        // Output
        let result = Solution::lowest_common_ancestor(root, p, q);

        // Expected Output
        let expected = Solution::vec_to_tree(vec![3, 5, 1, 6, 2, 0, 8, 0, 0, 7, 4]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example2() {
        // Input
        let nums = vec![3, 5, 1, 6, 2, 0, 8, 0, 0, 7, 4];
        let root = Solution::vec_to_tree(nums);
        let p = Solution::find_node_value(&root, 5);
        let q = Solution::find_node_value(&root, 4);

        // Output
        let result = Solution::lowest_common_ancestor(root, p, q);

        // Expected Output
        let expected = Solution::vec_to_tree(vec![5, 6, 2, 0, 0, 7, 4]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example3() {
        // Input
        let nums = vec![1, 2];
        let root = Solution::vec_to_tree(nums);
        let p = Solution::find_node_value(&root, 1);
        let q = Solution::find_node_value(&root, 2);

        // Output
        let result = Solution::lowest_common_ancestor(root, p, q);

        // Expected Output
        let expected = Solution::vec_to_tree(vec![1, 2]);

        assert_eq!(result, expected);
    }
}
