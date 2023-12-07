use crate::solution::Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; is_connected.len()];
        let mut count = 0;

        for i in 0..is_connected.len() {
            if !visited[i] {
                let mut stack = vec![i];
                while let Some(node1) = stack.pop() {
                    if !visited[node1] {
                        visited[node1] = true;
                        for node2 in 0..is_connected.len() {
                            if is_connected[node1][node2] == 1 {
                                stack.push(node2);
                            }
                        }
                    }
                }
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
