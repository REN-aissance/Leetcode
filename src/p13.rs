use std::cmp::Ordering;

pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut count = 0;
    let mut i = 0;
    let mut j = nums.len() - 1;

    while i < j {
        let sum = nums[i] + nums[j];
        match sum.cmp(&k) {
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
            Ordering::Equal => {
                count += 1;
                i += 1;
                j -= 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = max_operations(vec![1, 2, 3, 4], 5);
        assert_eq!(result, 2);
    }
    #[test]
    fn test2() {
        let result = max_operations(vec![3, 1, 3, 4, 3], 6);
        assert_eq!(result, 1);
    }
}
