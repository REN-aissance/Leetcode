use crate::solution::Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut out: Vec<_> = s.chars().collect();

        let mut indicies = s
            .char_indices()
            .filter_map(|(i1, v1)| if is_vowel(v1) { Some(i1) } else { None });

        while let Some(i1) = indicies.next() {
            if let Some(i2) = indicies.next_back() {
                out.swap(i1, i2);
            }
        }

        out.into_iter().collect()
    }
}

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'i' | 'u' | 'e' | 'o' | 'A' | 'I' | 'U' | 'E' | 'O')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = Solution::reverse_vowels("hello".to_string());
        assert_eq!(result, "holle".to_string());
    }
    #[test]
    fn test2() {
        let result = Solution::reverse_vowels("leetcode".to_string());
        assert_eq!(result, "leotcede");
    }
    #[test]
    fn test3() {
        let result = Solution::reverse_vowels("aA".to_string());
        assert_eq!(result, "Aa");
    }
    #[test]
    fn test4() {
        let result = Solution::reverse_vowels(".,".to_string());
        assert_eq!(result, ".,");
    }
}
