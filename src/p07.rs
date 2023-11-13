use crate::solution::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut out: Vec<_> = nums
            .iter()
            .scan(1, |acc, x| {
                let t = *acc;
                *acc *= x;
                Some(t)
            })
            .collect();

        nums.iter().enumerate().rev().fold(1, |acc, (i, x)| {
            out[i] *= acc;
            acc * x
        });

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = Solution::product_except_self(vec![1, 2, 3, 4]);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }
    #[test]
    fn test2() {
        let result = Solution::product_except_self(vec![-1, 1, 0, -3, 3]);
        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }
}
