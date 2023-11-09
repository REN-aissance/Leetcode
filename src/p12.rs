pub fn max_area(height: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut i = 0;
    let mut j = height.len() - 1;

    while i < j {
        a = a.max((j - i) as i32 * height[i].min(height[j]));
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(result, 49);
    }
    #[test]
    fn test2() {
        let result = max_area(vec![1, 1]);
        assert_eq!(result, 1);
    }
}
