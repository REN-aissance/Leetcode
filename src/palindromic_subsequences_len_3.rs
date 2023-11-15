use std::collections::HashSet;

use crate::solution::Solution;

/// Bonus challenge: Finds unique palindromic subsequences of length 3 of some string.
/// Resulted in faster than 100% of submissions every time. Data use is 50% average.
/// A cursory glance suggests this challenge can be done in O(n) time O(n) space like below,
/// or O(n^3) time O(1) space. (Trivial solution)
///
/// Very happy with this solution.
///
/// Learned that using a for loop with i or an
/// enumerated iterator is significantly slower than using a regular iterator.
/// Additionally it is faster to iterate through two len 26 arrays than using a
/// hashmap to maintain counts.

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut palindromes = HashSet::new();
        let s: Vec<_> = s.as_bytes().iter().map(|b| (b - b'a') as usize).collect();
        let mut left: [usize; 26] = [0; 26];
        let mut right: [usize; 26] = [0; 26];
        s.iter().skip(2).for_each(|&n| right[n] += 1);

        let mut s = s.as_slice().windows(3);
        while let Some(&[p, c, n]) = s.next() {
            left[p] += 1;
            for i in 0..26 {
                if left[i] > 0 && right[i] > 0 {
                    palindromes.insert((i << 10) + (c << 5) + i);
                }
            }
            right[n] -= 1;
        }

        palindromes.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use rand::distributions::Uniform;
    use std::time::{Duration, Instant};

    use rand::Rng;

    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("aabca");
        let result = Solution::count_palindromic_subsequence(s);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("adc");
        let result = Solution::count_palindromic_subsequence(s);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_3() {
        let s = String::from("bbcbaba");
        let result = Solution::count_palindromic_subsequence(s);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_4() {
        let s = String::from("ckafnafqo");
        let result = Solution::count_palindromic_subsequence(s);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[ignore]
    #[test]
    fn test_speed() {
        const ITERATIONS: usize = 100;
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(3..=10_000); //challenge condition
        let range = Uniform::new_inclusive('a', 'z'); //challenge condition
        let mut result = Vec::with_capacity(ITERATIONS);

        for _ in 0..ITERATIONS {
            let s = rng
                .clone()
                .sample_iter(range)
                .take(length)
                .collect::<String>();
            let start_time = Instant::now();
            let _ = Solution::count_palindromic_subsequence(s);
            let elapsed_time = start_time.elapsed();
            result.push(elapsed_time);
        }

        let total: Duration = result.iter().sum();
        let avg = total / result.len() as u32;

        dbg!(avg);
    }
}
