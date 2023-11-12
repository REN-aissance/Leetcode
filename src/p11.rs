use crate::solution::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.chars().peekable();
        for c in t.chars() {
            let sp = s.peek().copied();
            if Some(c) == sp {
                s.next();
            } else if sp.is_none() {
                return true;
            }
        }
        if s.peek().is_none() {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn test2() {
        let result = Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string());
        assert_eq!(result, false);
    }
}
