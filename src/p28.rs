use std::collections::VecDeque;

use crate::solution::Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant = VecDeque::new();
        let mut dire = VecDeque::new();
        senate.chars().enumerate().for_each(|(i, c)| match c {
            'R' => radiant.push_back(i),
            'D' => dire.push_back(i),
            _ => panic!(),
        });
        let mut n = radiant.len() + dire.len();
        while !radiant.is_empty() && !dire.is_empty() {
            if radiant.front().unwrap() < dire.front().unwrap() {
                dire.pop_front();
                radiant.pop_front();
                radiant.push_back(n);
            } else {
                radiant.pop_front();
                dire.pop_front();
                dire.push_back(n);
            }
            n += 1;
        }
        if dire.is_empty() {
            String::from("Radiant")
        } else if radiant.is_empty() {
            String::from("Dire")
        } else {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let senate = "RD".to_string();
        let result = Solution::predict_party_victory(senate);
        assert_eq!(result, "Radiant".to_string());
    }

    #[test]
    fn test_case2() {
        let senate = "RDD".to_string();
        let result = Solution::predict_party_victory(senate);
        assert_eq!(result, "Dire".to_string());
    }

    #[test]
    fn test_case3() {
        let senate = "R".to_string();
        let result = Solution::predict_party_victory(senate);
        assert_eq!(result, "Radiant".to_string());
    }

    #[test]
    fn test_case4() {
        let senate = "RRD".to_string();
        let result = Solution::predict_party_victory(senate);
        assert_eq!(result, "Radiant".to_string());
    }

    #[test]
    fn test_case5() {
        let senate = "RRDDD".to_string();
        let result = Solution::predict_party_victory(senate);
        assert_eq!(result, "Radiant".to_string());
    }
}
