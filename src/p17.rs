use crate::solution::Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        // gain.into_iter()
        //     .scan(0, |acc, n| {
        //         let t = *acc;
        //         *acc += n;
        //         Some(t)
        //     })
        //     .max()
        //     .unwrap()

        gain.iter()
            .fold((0, 0), |(mut max, mut cur), delta| {
                cur += delta;
                max = std::cmp::max(cur, max);
                (max, cur)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use rand::Rng;

    use super::*;

    #[test]
    fn test_example1() {
        let gain = vec![-5, 1, 5, 0, -7];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_example2() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_speed() {
        // Generate a long vector with random binary values
        let size = 1000000; // You can adjust the size as per your requirement
        let iterations = 100;

        // Time the execution of the function
        let mut result = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let nums = generate_random_vec(size);
            let start_time = Instant::now();
            let _ = Solution::largest_altitude(nums);
            let elapsed_time = start_time.elapsed();
            result.push(elapsed_time);
        }

        let total = result.iter().fold(Duration::ZERO, |acc, d| acc + *d);
        let avg = total / result.len() as u32;

        dbg!(avg);
    }

    fn generate_random_vec(size: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(-100..=100)).collect()
    }
}
