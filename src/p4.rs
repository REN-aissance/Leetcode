use crate::solution::Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let flowerbed = [vec![0], flowerbed, vec![0]].concat();
        let mut places = 0;

        let mut windows = flowerbed.windows(3);
        while let Some([a, b, c]) = windows.next() {
            if a == &0 && b == &0 && c == &0 {
                places += 1;
                windows.next();
            }
        }

        places == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1);
        assert!(result);
    }
    #[test]
    fn test2() {
        let result = Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2);
        assert!(!result);
    }
    #[test]
    fn test3() {
        let result = Solution::can_place_flowers(vec![1, 0, 0, 0, 1, 0, 0], 2);
        assert!(result);
    }
}
