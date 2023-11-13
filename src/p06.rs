use crate::solution::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let s = String::from("the sky is blue");
        let result = Solution::reverse_words(s);
        assert_eq!(result, String::from("blue is sky the"));
    }

    #[test]
    fn example_2() {
        let s = String::from("  hello world  ");
        let result = Solution::reverse_words(s);
        assert_eq!(result, String::from("world hello"));
    }

    #[test]
    fn example_3() {
        let s = String::from("a good   example");
        let result = Solution::reverse_words(s);
        assert_eq!(result, String::from("example good a"));
    }
}
