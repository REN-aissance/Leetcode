use crate::solution::Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut zeroes = k as i32 - (nums.get(0..k).unwrap().iter().sum::<i32>());
        let mut i = 0;
        let mut j = k;
        let k = k as i32;
        let mut max = j - i;

        while j < nums.len() + 1 {
            while zeroes < k {
                match nums.get(j) {
                    Some(0) => zeroes += 1,
                    None => break,
                    _ => (),
                }
                j += 1;
            }
            while zeroes > k {
                if nums[i] == 0 {
                    zeroes -= 1;
                }
                i += 1;
            }
            max = std::cmp::max(j - i, max);
            if let Some(0) = nums.get(j) {
                zeroes += 1;
            }
            j += 1;
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 2;
        let result = Solution::longest_ones(nums, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example2() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let k = 3;
        let result = Solution::longest_ones(nums, k);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_example3() {
        let nums = vec![0, 0, 0, 1];
        let k = 4;
        let result = Solution::longest_ones(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example4() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let k = 0;
        let result = Solution::longest_ones(nums, k);
        assert_eq!(result, 4);
    }
}
