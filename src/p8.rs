pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut first = i32::MAX;
    let mut second = i32::MAX;
    for n in nums {
        if n <= first {
            first = n;
        } else if n <= second {
            second = n;
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = increasing_triplet(vec![1, 2, 3, 4, 5]);
        assert_eq!(result, true);
    }
    #[test]
    fn test2() {
        let result = increasing_triplet(vec![5, 4, 3, 2, 1]);
        assert_eq!(result, false);
    }
    #[test]
    fn test4() {
        let result = increasing_triplet(vec![2, 1, 5, 0, 4, 6]);
        assert_eq!(result, true);
    }
}
