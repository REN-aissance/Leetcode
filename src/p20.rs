use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut count_map = HashMap::new();
        for &n in arr.iter() {
            *count_map.entry(n).or_insert(0) += 1;
        }
        let mut occurrences = HashSet::new();
        for &count in count_map.values() {
            if !occurrences.insert(count) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        assert_eq!(Solution::unique_occurrences(arr), true);
    }

    #[test]
    fn test_case2() {
        let arr = vec![1, 2];
        assert_eq!(Solution::unique_occurrences(arr), false);
    }

    #[test]
    fn test_case3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert_eq!(Solution::unique_occurrences(arr), true);
    }
}
