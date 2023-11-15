use crate::solution::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut stack = Vec::with_capacity(rooms.len());
        stack.push(0);
        let mut visited = HashSet::new();

        while let Some(cur) = stack.pop() {
            visited.insert(cur);
            for key in &rooms[cur] {
                let key = *key as usize;
                if !visited.contains(&key) {
                    stack.push(key);
                }
            }
        }

        visited.len() == rooms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rooms_1() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        assert!(Solution::can_visit_all_rooms(rooms));
    }

    #[test]
    fn rooms_2() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        assert!(!Solution::can_visit_all_rooms(rooms));
    }
}
