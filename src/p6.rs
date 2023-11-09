pub fn reverse_words(s: String) -> String {
    s.split_ascii_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = reverse_words("hello".to_string());
        assert_eq!(result, "holle".to_string());
    }
    #[test]
    fn test2() {
        let result = reverse_words("leetcode".to_string());
        assert_eq!(result, "leotcede");
    }
    #[test]
    fn test3() {
        let result = reverse_words("aA".to_string());
        assert_eq!(result, "Aa");
    }
    #[test]
    fn test4() {
        let result = reverse_words(".,".to_string());
        assert_eq!(result, ".,");
    }
}
