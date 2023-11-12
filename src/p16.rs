use crate::solution::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut left_ptr = 0;
        let mut right_ptr = 0;
        let mut sum = 0;
        let mut max = 0;

        while let Some(num) = nums.get(right_ptr) {
            sum += num;
            let interval = right_ptr - left_ptr;
            match interval.cmp(&(sum as usize)) {
                Ordering::Greater => {
                    max = max.max(sum - 1);
                    sum -= nums[left_ptr];
                    left_ptr += 1;
                }
                _ => max = max.max(sum),
            }
            right_ptr += 1;
        }
        //Account for the case where there are no zeroes
        if max == nums.len() as i32 {
            max -= 1;
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::time::{Duration, Instant};

    #[test]
    fn test_example1() {
        let nums = vec![1, 1, 0, 1];
        let result = Solution::longest_subarray(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_example2() {
        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        let result = Solution::longest_subarray(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_example3() {
        let nums = vec![1, 1, 1];
        let result = Solution::longest_subarray(nums);
        assert_eq!(result, 2);
    }

    #[test]
    #[ignore]
    fn test_speed() {
        // Generate a long vector with random binary values
        let size = 1000000; // You can adjust the size as per your requirement

        // Time the execution of the function
        let mut result = Vec::new();

        for _ in 0..100 {
            let nums = generate_random_binary_vector(size);
            let start_time = Instant::now();
            let _ = Solution::longest_subarray(nums);
            let elapsed_time = start_time.elapsed();
            result.push(elapsed_time);
        }

        let total = result.iter().fold(Duration::ZERO, |acc, d| acc + *d);
        let avg = total / result.len() as u32;

        dbg!(avg);
    }

    fn generate_random_binary_vector(size: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(0..=1)).collect()
    }
}
