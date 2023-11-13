use crate::p33::TreeNode;
use crate::solution::Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type OptNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn max_level_sum(root: OptNode) -> i32 {
        if let Some(node) = root {
            let mut out = vec![];
            let mut q = VecDeque::new();
            q.push_back(node);
            while !q.is_empty() {
                let level_size = q.len();
                let mut level_sum = 0;
                for _ in 0..level_size {
                    if let Some(current_node) = q.pop_front() {
                        let val = current_node.borrow().val;
                        level_sum += val;
                        if let Some(left) = &current_node.borrow().left {
                            q.push_back(left.clone());
                        }
                        if let Some(right) = &current_node.borrow().right {
                            q.push_back(right.clone());
                        }
                    }
                }
                out.push(level_sum);
            }
            return (1..out.len() + 1)
                .zip(out)
                .rev()
                .max_by(|x, y| x.1.cmp(&y.1))
                .unwrap()
                .0 as i32;
        }
        panic!()
    }

    pub fn vec_with_zeroes(nums: Vec<Option<i32>>) -> OptNode {
        if nums.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
        let mut q = VecDeque::new();
        q.push_back(root.clone());

        for chunk in nums[1..].chunks(2) {
            let cur = q.pop_front().unwrap();
            let (l, r) = match chunk {
                [left, right] => (*left, *right),
                _ => return None,
            };
            if let Some(l) = l {
                let left_node = Rc::new(RefCell::new(TreeNode::new(l)));
                cur.borrow_mut().left = Some(left_node.clone());
                q.push_back(left_node);
            }
            if let Some(r) = r {
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
        let input = vec![Some(1), Some(7), Some(0), Some(7), Some(-8), None, None];
        let root = Solution::vec_with_zeroes(input);
        let result = Solution::max_level_sum(root);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example2() {
        let input = vec![
            Some(989),
            None,
            Some(10250),
            Some(98693),
            Some(-89388),
            None,
            None,
            None,
            Some(-32127),
        ];
        let root = Solution::vec_with_zeroes(input);
        let result = Solution::max_level_sum(root);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example3() {
        let input = vec![
            Some(1),
            Some(1),
            Some(0),
            Some(7),
            Some(-8),
            Some(-7),
            Some(9),
        ];
        let root = Solution::vec_with_zeroes(input);
        let result = Solution::max_level_sum(root);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example4() {
        let input = vec![Some(1), Some(2), Some(3)];
        let root = Solution::vec_with_zeroes(input);
        let result = Solution::max_level_sum(root);
        let expected = 2;
        assert_eq!(result, expected);
    }
}
