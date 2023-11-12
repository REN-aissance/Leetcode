use crate::solution::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_nz = 0;
        for cur in 0..nums.len() {
            let cur = nums[cur];
            if cur != 0 {
                nums[last_nz] = cur;
                last_nz += 1;
            }
        }
        nums.iter_mut().skip(last_nz).for_each(|n| *n = 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1, 3, 12, 0, 0]);
    }
    #[test]
    fn test2() {
        let mut v = vec![0];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![0]);
    }
}
