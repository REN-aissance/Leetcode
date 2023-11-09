use crate::solution::Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum_left = 0;
        let sum = nums.iter().sum();

        for (i, n) in nums.iter().enumerate() {
            if (sum_left << 1) + n == sum {
                return i as i32;
            }
            sum_left += n;
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }

    #[test]
    fn test_example4() {
        assert_eq!(Solution::pivot_index(vec![-1, -1, 0, 0, -1, -1]), 2);
    }
}
