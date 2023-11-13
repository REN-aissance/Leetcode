use crate::solution::Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut currentchar = ' ';
        let mut count = 0;
        let mut ptr = 0;
        for i in 0..=chars.len() {
            let c = chars.get(i).copied();
            if c == Some(currentchar) {
                count += 1;
            } else {
                if count > 1 {
                    for nc in count.to_string().chars() {
                        *chars.get_mut(ptr).unwrap() = nc;
                        ptr += 1;
                    }
                    for c in chars.get_mut(ptr..i).unwrap() {
                        *c = ' ';
                    }
                }
                count = 1;
                currentchar = c.unwrap_or(' ');
                ptr = i + 1;
            }
        }
        chars.retain(|c| c != &' ');
        chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut v = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let result = Solution::compress(&mut v);
        assert_eq!(result, 6);
        assert_eq!(v, vec!['a', '2', 'b', '2', 'c', '3']);
    }
    #[test]
    fn test2() {
        let mut v = vec!['a'];
        let result = Solution::compress(&mut v);
        assert_eq!(result, 1);
        assert_eq!(v, vec!['a']);
    }
    #[test]
    fn test3() {
        let mut v = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let result = Solution::compress(&mut v);
        assert_eq!(result, 4);
        assert_eq!(v, vec!['a', 'b', '1', '2']);
    }
    #[test]
    fn test4() {
        let mut v = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
        let result = Solution::compress(&mut v);
        assert_eq!(result, 6);
        assert_eq!(v, vec!['a', '3', 'b', '2', 'a', '2']);
    }
}
