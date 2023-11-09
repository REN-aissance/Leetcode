use crate::solution::Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        s.chars()
            .fold(vec![], |mut stack, c| {
                if c == '*' {
                    stack.pop();
                } else {
                    stack.push(c);
                }
                stack
            })
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        // Test Case 1
        let input = String::from("leet**cod*e");
        let expected_output = "lecoe";
        assert_eq!(Solution::remove_stars(input), expected_output);
    }

    #[test]
    fn test_case_2() {
        // Test Case 2
        let input = String::from("erase*****");
        let expected_output = "";
        assert_eq!(Solution::remove_stars(input), expected_output);
    }

    #[test]
    fn test_case_3() {
        // Test Case 4
        let input = String::from("a*b*c*d*e*f*g*h*i*j*");
        let expected_output = "";
        assert_eq!(Solution::remove_stars(input), expected_output);
    }

    #[test]
    fn test_case_4() {
        // Test Case 5
        let input = String::from("*a*b*c*d*e*f*g*h*i*j*");
        let expected_output = "";
        assert_eq!(Solution::remove_stars(input), expected_output);
    }

    #[test]
    fn test_case_5() {
        // Test Case 6
        let input = String::from("****");
        let expected_output = "";
        assert_eq!(Solution::remove_stars(input), expected_output);
    }

    #[test]
    fn test_case_6() {
        // Test Case 7
        let input = String::from("a*b*c*d*e*f*g*h*i*j*k*l*m*n*o*p*q*r*s*t*u*v*w*x*y*z*");
        let expected_output = "";
        assert_eq!(Solution::remove_stars(input), expected_output);
    }
}
