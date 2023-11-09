use crate::solution::Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;

        const VOWELS: [u8; 5] = [b'a', b'i', b'u', b'e', b'o'];
        let mut count = s.iter().take(k).filter(|b| VOWELS.contains(b)).count();
        let mut max = count;

        for i in k..s.len() {
            count += is_vowel(&s[i]);
            count -= is_vowel(&s[i - k]);
            max = std::cmp::max(max, count);
        }

        max as i32
    }
}

fn is_vowel(b: &u8) -> usize {
    match b {
        b'a' | b'i' | b'u' | b'e' | b'o' => 1,
        _ => 0,
    }
}

//Quick solution. Too slow for the challenge
// impl Solution {
//     pub fn max_vowels(s: String, k: i32) -> i32 {
//         s.chars()
//             .collect::<Vec<_>>()
//             .windows(k as usize)
//             .map(|w| w.iter().filter(|c| is_vowel(c)).count())
//             .max()
//             .unwrap() as i32
//     }
// }
// fn is_vowel(c: &char) -> bool {
//     matches!(c, 'a' | 'i' | 'u' | 'e' | 'o')
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example4() {
        let count = Solution::max_vowels("ibpbhixfiouhdljnjfflpapptrxgcomvnb".to_string(), 33); // Your vowel counting function here
        assert_eq!(count, 7);
    }

    #[test]
    fn example1() {
        let s = String::from("abciiidef");
        let k = 3;
        assert_eq!(Solution::max_vowels(s, k), 3);
    }

    #[test]
    fn example2() {
        let s = String::from("aeiou");
        let k = 2;
        assert_eq!(Solution::max_vowels(s, k), 2);
    }

    #[test]
    fn example3() {
        let s = String::from("leetcode");
        let k = 3;
        assert_eq!(Solution::max_vowels(s, k), 2);
    }
}
