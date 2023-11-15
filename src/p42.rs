use crate::p33::TreeNode;
use crate::solution::Solution;
use std::cell::RefCell;
use std::rc::Rc;
type OptNode = Option<Rc<RefCell<TreeNode>>>;
type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn delete_node(root: OptNode, key: i32) -> OptNode {
        Solution::helper(&root, key)
    }
    fn helper(node: &OptNode, key: i32) -> OptNode {
        let n = node.clone()?;
        let val = n.borrow().val;
        match val.cmp(&key) {
            std::cmp::Ordering::Greater => {
                let l = Solution::helper(&n.borrow().left, key);
                n.borrow_mut().left = l;
            }
            std::cmp::Ordering::Less => {
                let r = Solution::helper(&n.borrow().right, key);
                n.borrow_mut().right = r;
            }
            std::cmp::Ordering::Equal => {
                if n.borrow().left.is_none() {
                    return n.borrow().right.clone();
                }
                if n.borrow().right.is_none() {
                    return n.borrow().left.clone();
                }
                let next = Solution::next(&n.borrow().right);
                if let Some(val) = next {
                    let r = Solution::helper(&n.borrow().right, val);
                    n.borrow_mut().val = val;
                    n.borrow_mut().right = r;
                }
            }
        }
        node.clone()
    }
    fn next(node: &OptNode) -> Option<i32> {
        if let Some(n) = node {
            if n.borrow().left.is_some() {
                Solution::next(&n.borrow().left)
            } else {
                Some(n.borrow().val)
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
    fn test_example_1() {
        let root = Solution::vec_with_zeroes(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(7),
        ]);
        let result = Solution::delete_node(root, 3);
        let expected = Solution::vec_with_zeroes(vec![
            Some(5),
            Some(4),
            Some(6),
            Some(2),
            None,
            None,
            Some(7),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let root = Solution::vec_with_zeroes(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(7),
        ]);
        let result = Solution::delete_node(root, 0);
        let expected = Solution::vec_with_zeroes(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(7),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_3() {
        let root = Solution::vec_with_zeroes(vec![]);
        let result = Solution::delete_node(root, 0);
        let expected = Solution::vec_with_zeroes(vec![]);
        assert_eq!(result, expected);
    }
}
