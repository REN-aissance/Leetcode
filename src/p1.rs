pub fn merge_alternatively(word1: String, word2: String) -> String {
    let repeat = word1.len().max(word2.len());
    let mut w1 = word1.chars();
    let mut w2 = word2.chars();
    let mut acc = String::new();

    for _ in 0..repeat {
        if let Some(s1) = w1.next() {
            acc.push_str(&s1.to_string());
        }
        if let Some(s2) = w2.next() {
            acc.push_str(&s2.to_string());
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = merge_alternatively("Hello".to_string(), "Hello".to_string());
        assert_eq!(result, "HHeelllloo".to_string());
    }
}
