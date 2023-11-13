use crate::p33::TreeNode;
use crate::solution::Solution;
use std::cell::RefCell;
use std::rc::Rc;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn right_side_view(root: OptNode) -> Vec<i32> {
        let mut out = vec![];
        rview(&root, &mut out, 0);
        return out;

        fn rview(root: &OptNode, out: &mut Vec<i32>, depth: usize) {
            if root.is_some() {
                if depth == out.len() {
                    out.push(root.as_ref().unwrap().borrow().val);
                }
                rview(&root.as_ref().unwrap().borrow().right, out, depth + 1);
                rview(&root.as_ref().unwrap().borrow().left, out, depth + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let root = Solution::vec_to_tree(vec![1, 2, 3, 0, 5, 0, 4]);
        let result = Solution::right_side_view(root);
        let expected = vec![1, 3, 4];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example2() {
        let root = Solution::vec_to_tree(vec![1, 0, 3]);
        let result = Solution::right_side_view(root);
        let expected = vec![1, 3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example3() {
        let root = None;
        let result = Solution::right_side_view(root);
        let expected = Vec::<i32>::new();
        assert_eq!(result, expected);
    }
}
