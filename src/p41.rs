use crate::p33::TreeNode;
use crate::solution::Solution;
use std::cell::RefCell;
use std::rc::Rc;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn search_bst(root: OptNode, val: i32) -> OptNode {
        let mut cur_node = root?;
        loop {
            let cur_val = cur_node.clone().borrow().val;
            match cur_val.cmp(&val) {
                std::cmp::Ordering::Less => cur_node = cur_node.clone().borrow().right.clone()?,
                std::cmp::Ordering::Greater => cur_node = cur_node.clone().borrow().left.clone()?,
                std::cmp::Ordering::Equal => return Some(cur_node),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_bst_1() {
        let root = Solution::vec_to_tree(vec![4, 2, 7, 1, 3]);
        let result = Solution::search_bst(root, 2);
        let expected = Solution::vec_to_tree(vec![2, 1, 3]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_search_bst_2() {
        let root = Solution::vec_to_tree(vec![4, 2, 7, 1, 3]);
        let result = Solution::search_bst(root, 5);
        let expected = None;
        assert_eq!(result, expected);
    }
}
