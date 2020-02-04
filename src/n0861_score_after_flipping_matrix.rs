/**
 * [861] Score After Flipping Matrix
 *
 * We have a two dimensional matrix A where each value is 0 or 1.
 *
 * A move consists of choosing any row or column, and toggling each value in that row or column: changing all 0s to 1s, and all 1s to 0s.
 *
 * After making any number of moves, every row of this matrix is interpreted as a binary number, and the score of the matrix is the sum of these numbers.
 *
 * Return the highest possible score.
 *
 *  
 *
 * <ol>
 * </ol>
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">[[0,0,1,1],[1,0,1,0],[1,1,0,0]]</span>
 * Output: <span id="example-output-1">39</span>
 * Explanation:
 * Toggled to <span id="example-input-1-1">[[1,1,1,1],[1,0,0,1],[1,1,1,1]].
 * 0b1111 + 0b1001 + 0b1111 = 15 + 9 + 15 = 39</span>
 *
 *  
 *
 * Note:
 *
 * <ol>
 *     1 <= A.length <= 20
 *     1 <= A[0].length <= 20
 *     A[i][j] is 0 or 1.
 * </ol>
 * </div>
 *
 */
pub struct Solution {}

// submission codes start here

use std::cmp;

impl Solution {
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let row = a.len();
        let col = unsafe { a.get_unchecked(0).len() };
        let mut score = (1 << (col - 1)) * row;
        for j in 1..col {
            let ones: usize = a
                .iter()
                .map(|r| unsafe { r.get_unchecked(0) == r.get_unchecked(j) } as usize)
                .sum();
            score += cmp::max(ones, row - ones) << (col - j - 1);
        }
        score as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_861() {
        assert_eq!(
            Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
            39
        );
        assert_eq!(
            Solution::matrix_score(vec![
                vec![1, 0, 0, 0, 1, 0, 1, 0],
                vec![1, 0, 1, 0, 0, 1, 1, 1],
                vec![1, 1, 1, 1, 0, 0, 0, 1],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                vec![0, 1, 1, 1, 0, 0, 0, 1],
                vec![1, 1, 0, 1, 1, 1, 1, 0],
                vec![1, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 1, 0, 0, 1],
                vec![0, 1, 1, 0, 0, 0, 1, 0]
            ]),
            1870
        );
    }
}
