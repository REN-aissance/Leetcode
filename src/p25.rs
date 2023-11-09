use core::panic;

use crate::solution::Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = Vec::new();
        let mut output = String::new();
        let mut count = 0;

        for c in s.chars() {
            match c {
                '0'..='9' => {
                    count *= 10;
                    count += c.to_digit(10).unwrap() as usize;
                }
                'a'..='z' => {
                    output.push(c);
                }
                ']' => {
                    let (prev_count, prev_string) = stack.pop().unwrap();
                    output = prev_string + output.repeat(prev_count).as_str();
                }
                '[' => {
                    stack.push((count, std::mem::take(&mut output)));
                    count = 0;
                }
                _ => panic!("Invalid input"),
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string_example1() {
        let input = "3[a]2[bc]".to_string();
        let output = "aaabcbc".to_string();
        assert_eq!(Solution::decode_string(input), output);
    }

    #[test]
    fn test_decode_string_example2() {
        let input = "3[a2[c]]".to_string();
        let output = "accaccacc".to_string();
        assert_eq!(Solution::decode_string(input), output);
    }

    #[test]
    fn test_decode_string_example3() {
        let input = "2[abc]3[cd]ef".to_string();
        let output = "abcabccdcdcdef".to_string();
        assert_eq!(Solution::decode_string(input), output);
    }

    #[test]
    fn test_decode_string_additional1() {
        let input = "10[a]".to_string();
        let output = "aaaaaaaaaa".to_string();
        assert_eq!(Solution::decode_string(input), output);
    }

    #[test]
    fn test_decode_string_additional2() {
        let input = "2[3[a]b]".to_string();
        let output = "aaabaaab".to_string();
        assert_eq!(Solution::decode_string(input), output);
    }
}
