pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap();
    candies.iter().map(|c| c + extra_candies >= max).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = kids_with_candies(vec![2, 3, 5, 1, 3], 3);
        assert_eq!(result, vec![true, true, true, false, true]);
    }
    #[test]
    fn test2() {
        let result = kids_with_candies(vec![4, 2, 1, 1, 2], 1);
        assert_eq!(result, vec![true, false, false, false, false]);
    }
    #[test]
    fn test3() {
        let result = kids_with_candies(vec![12, 1, 12], 10);
        assert_eq!(result, vec![true, false, true]);
    }
}
