/**
 * [1386] Shift 2D Grid
 *
 * Given a 2D grid of size m x n and an integer k. You need to shift the grid k times.
 * In one shift operation:
 *
 * 	Element at grid[i][j] becomes at grid[i][j + 1].
 * 	Element at grid[i][n - 1] becomes at grid[i + 1][0].
 * 	Element at grid[n - 1][n - 1] becomes at grid[0][0].
 *
 * Return the 2D grid after applying shift operation k times.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/05/e1.png" style="width: 400px; height: 178px;" />
 * Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 1
 * Output: [[9,1,2],[3,4,5],[6,7,8]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/05/e2.png" style="width: 400px; height: 166px;" />
 * Input: grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], k = 4
 * Output: [[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]
 *
 * Example 3:
 *
 * Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 9
 * Output: [[1,2,3],[4,5,6],[7,8,9]]
 *
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m <= 50
 * 	1 <= n <= 50
 * 	-1000 <= grid[i][j] <= 1000
 * 	0 <= k <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = unsafe { grid.get_unchecked(0).len() };
        let total = rows * cols;
        let k = (k as usize) % total;
        if k == 0 {
            return grid;
        }
        let mut cache = vec![0; total];
        let mut i = 0usize;
        for row in &grid {
            for n in row {
                unsafe {
                    *(cache.get_unchecked_mut((i + k) % total)) = *n;
                }
                i += 1;
            }
        }

        // reform the grid
        let mut cur = cache.iter();
        let mut grid = vec![vec![0; cols]; rows];
        unsafe {
            for i in 0..rows {
                let mut row = grid.get_unchecked_mut(i);
                for j in 0..cols {
                    *(row.get_unchecked_mut(j)) = *cur.next().unwrap();
                }
            }
        }
        grid
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1386() {
        assert_eq!(
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
        assert_eq!(
            Solution::shift_grid(
                vec![
                    vec![3, 8, 1, 9],
                    vec![19, 7, 2, 5],
                    vec![4, 6, 11, 10],
                    vec![12, 0, 21, 13]
                ],
                4
            ),
            vec![
                vec![12, 0, 21, 13],
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10]
            ]
        );
        assert_eq!(
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        );
    }
}
