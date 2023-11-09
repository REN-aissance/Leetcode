use std::collections::HashSet;

use crate::solution::Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1 = nums1.into_iter().collect::<HashSet<_>>();
        let nums2 = nums2.into_iter().collect::<HashSet<_>>();
        let o1 = nums1.difference(&nums2).copied().collect();
        let o2 = nums2.difference(&nums1).copied().collect();
        vec![o1, o2]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let output = vec![vec![1, 3], vec![4, 6]];
        assert_eq!(Solution::find_difference(nums1, nums2), output);
    }

    #[test]
    fn test_example2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let output = vec![vec![3], vec![]];
        assert_eq!(Solution::find_difference(nums1, nums2), output);
    }
}
