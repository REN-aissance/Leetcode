use crate::solution::Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![asteroids[0]];
        for a in asteroids.into_iter().skip(1) {
            handle_asteroid(&mut stack, a);
        }
        stack
    }
}

fn handle_asteroid(stack: &mut Vec<i32>, asteroid: i32) {
    if let Some(last) = stack.last() {
        if asteroid < 0 && last > &0 {
            match asteroid.abs().cmp(last) {
                std::cmp::Ordering::Equal => {
                    stack.pop();
                }
                std::cmp::Ordering::Greater => {
                    stack.pop();
                    handle_asteroid(stack, asteroid);
                }
                _ => (),
            }
        } else {
            stack.push(asteroid);
        }
    } else {
        stack.push(asteroid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        // Test Case 1
        let asteroids = vec![5, 10, -5];
        let expected_output = vec![5, 10];
        assert_eq!(Solution::asteroid_collision(asteroids), expected_output);
    }

    #[test]
    fn test_case_2() {
        // Test Case 2
        let asteroids = vec![8, -8];
        let expected_output: Vec<i32> = Vec::new();
        assert_eq!(Solution::asteroid_collision(asteroids), expected_output);
    }

    #[test]
    fn test_case_3() {
        // Test Case 3
        let asteroids = vec![10, 2, -5];
        let expected_output = vec![10];
        assert_eq!(Solution::asteroid_collision(asteroids), expected_output);
    }

    #[test]
    fn test_case_4() {
        // Test Case 4
        let asteroids = vec![-2, -1, 1, 2];
        let expected_output = vec![-2, -1, 1, 2];
        assert_eq!(Solution::asteroid_collision(asteroids), expected_output);
    }
}
