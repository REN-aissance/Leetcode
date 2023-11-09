use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

///This would be a good solution in the case that the two words
/// had an unknown number of symbols, however, as the number of
/// letters in the problem is limited to a-z using an array is better

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        //If words do not have the same numbers of characters they are not 'similar'
        if word1.len() != word2.len() {
            return false;
        }
        //Create a frequency map of characters
        let f1 = frequency_map(word1.chars());
        let f2 = frequency_map(word2.chars());

        //If the words do not have the same charset, they are not 'similar'
        if f1.keys().collect::<HashSet<_>>() != f2.keys().collect() {
            return false;
        }

        //Switching the counts of two letters effectively fulfills the letter swapping rule
        //This can be repeated infinitely, meaning that if the frequency of counts is the same,
        //the words are 'similar'
        if frequency_map(f1.values()) != frequency_map(f2.values()) {
            return false;
        }

        true
    }

    // pub fn close_strings(word1: String, word2: String) -> bool {
    //     let (mut map1, mut map2) = ([0; 26], [0; 26]);
    //     let char_offset = 'a' as usize
    //     word1.chars().for_each(|c| map1[c as usize - char_offset] += 1);
    //     word2.chars().for_each(|c| map2[c as usize - char_offset] += 1);
    //     for i in 0..26 {
    //         if (map1[i] == 0 && map2[i] != 0) || (map1[i] != 0 && map2[i] == 0) {
    //             return false;
    //         }
    //     }
    //     map1.sort();
    //     map2.sort();
    //     map1 == map2
    // }
}

fn frequency_map<I, T>(s: I) -> HashMap<T, usize>
where
    T: Eq + PartialEq + std::hash::Hash,
    I: Iterator<Item = T>,
{
    s.fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|v| *v += 1).or_insert(1);
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::close_strings("abc".to_string(), "bca".to_string()),
            true
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::close_strings("a".to_string(), "aa".to_string()),
            false
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::close_strings("cabbba".to_string(), "abbccc".to_string()),
            true
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            Solution::close_strings("uau".to_string(), "ssx".to_string()),
            false
        );
    }
}
