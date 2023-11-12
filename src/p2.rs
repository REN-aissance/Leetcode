use crate::solution::Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str2.len() > str1.len() {
            return Self::gcd_of_strings(str2, str1);
        };

        for i in (1..(str2.len() + 1)).rev() {
            if str2.len() % i != 0 {
                continue;
            }
            let prefix = &str2[0..i];
            if divides_string(&str1, prefix) && divides_string(&str2, prefix) {
                return prefix.to_string();
            }
        }

        String::new()
    }
}

fn divides_string(str: &str, prefix: &str) -> bool {
    if str.len() % prefix.len() != 0 {
        return false;
    }
    let mut prefix = prefix.chars().cycle();
    for c in str.chars() {
        if c != prefix.next().unwrap() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s1 = &"ABCABCABC";
        let s2 = &"ABCABC";
        let result = Solution::gcd_of_strings(s1.to_string(), s2.to_string());
        assert_eq!(result, "ABC".to_string());
    }
}
