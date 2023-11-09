use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let n = grid.len();
        let mut map = HashMap::with_capacity(n * 2);

        //Add rows to map
        for row in grid.iter() {
            map.entry(row.clone()).and_modify(|v| *v += 1).or_insert(1);
        }
        //Transpose grid (glad it's square)
        for j in 0..n {
            for i in (j + 1)..n {
                let t = grid[j][i];
                grid[j][i] = grid[i][j];
                grid[i][j] = t;
            }
        }
        //Add rows number of pairs for each column to some accumulator
        grid.into_iter().filter_map(|row| map.get(&row)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        assert_eq!(Solution::equal_pairs(grid), 1);
    }

    #[test]
    fn test_case2() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        assert_eq!(Solution::equal_pairs(grid), 3);
    }

    #[test]
    fn test_case3() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::equal_pairs(grid), 0);
    }

    #[test]
    fn test_case4() {
        let grid = vec![vec![13, 13], vec![13, 13]];
        assert_eq!(Solution::equal_pairs(grid), 4);
    }
}
