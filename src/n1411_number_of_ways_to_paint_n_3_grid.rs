/**
 * [1411] Number of Ways to Paint N × 3 Grid
 *
 * You have a `grid` of size `n x 3` and you want to paint each cell of the grid with exactly one of the three colours: Red, Yellow or Green while making sure that no two adjacent cells have the same colour (i.e no two cells that share vertical or horizontal sides have the same colour).
 *
 * You are given `n` the number of rows of the grid.
 *
 * Return the number of ways you can paint this `grid`. As the answer may grow large, the answer must be computed modulo `10^9 + 7`.
 *
 *  
 * Example 1:
 *
 *
 * Input: n = 1
 * Output: 12
 * Explanation: There are 12 possible way to paint the grid as shown:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/03/26/e1.png" style="width: 450px; height: 289px;" />
 *
 *
 * Example 2:
 *
 *
 * Input: n = 2
 * Output: 54
 *
 *
 * Example 3:
 *
 *
 * Input: n = 3
 * Output: 246
 *
 *
 * Example 4:
 *
 *
 * Input: n = 7
 * Output: 106494
 *
 *
 * Example 5:
 *
 *
 * Input: n = 5000
 * Output: 30228214
 *
 *
 *  
 * Constraints:
 *
 *
 *     `n == grid.length`
 *     `grid[i].length == 3`
 *     `1 <= n <= 5000`
 *
 * Problem link: https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/
 * Discuss link: https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn num_of_ways_solution_b(n: i32) -> i32 {
        let modulo = 1_000_000_007;
        /// Next row patterns
        /// The patterns follow the map: {'grg': 0, 'gry': 1, 'gyg': 2, 'gyr': 3, 'rgr': 4, 'rgy': 5, 'ryg': 6, 'ryr': 7, 'ygr': 8, 'ygy': 9, 'yrg': 10, 'yry': 11}
        let next_patterns = [
            [4, 9, 8, 7, 5],
            [6, 4, 7, 8, 12],
            [4, 9, 8, 11, 5],
            [11, 9, 5, 10, 12],
            [10, 1, 0, 2, 11],
            [3, 0, 10, 2, 12],
            [8, 11, 9, 1, 12],
            [9, 1, 10, 0, 11],
            [6, 0, 1, 2, 12],
            [3, 6, 7, 0, 2],
            [3, 4, 5, 7, 12],
            [3, 4, 6, 7, 2],
            [12, 12, 12, 12, 12], // a dead end
        ];
        let mut todo = [1; 13];
        for i in 2..(1 + n as usize) {
            let mut todo2 = [0; 13]; // 12 good
            for (color_patten, &count) in todo[..12].iter().enumerate() {
                for &pattern in next_patterns[color_patten].iter() {
                    todo2[pattern] = (todo2[pattern] + count) % modulo;
                }
            }
            todo = todo2;
        }
        let mut result = 0;
        for &n in todo[..12].iter() {
            result = (result + n) % modulo;
        }
        result % modulo
    }

    pub fn num_of_ways_solution_a(n: i32) -> i32 {
        // ref: https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/discuss/574923/JavaC%2B%2BPython-DP-O(1)-Space
        let (mut a121, mut a123, modulo) = (6usize, 6usize, 1_000_000_007usize);
        for _ in 1..(n as usize) {
            let next_a121 = (a121 * 3 + a123 * 2) % modulo;
            let next_a123 = (a121 * 2 + a123 * 2) % modulo;
            a121 = next_a121;
            a123 = next_a123;
        }
        ((a121 + a123) % modulo) as i32
    }

    pub fn num_of_ways_solution_s(n: i32) -> i32 {
        // ref: https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/discuss/575485/C++Python-O(logN)-Time
        0
    }

    pub fn num_of_ways(n: i32) -> i32 {
        Solution::num_of_ways_solution_a(n)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1411() {
        assert_eq!(Solution::num_of_ways(1), 12);
        assert_eq!(Solution::num_of_ways(2), 54);
        assert_eq!(Solution::num_of_ways(3), 246);
        assert_eq!(Solution::num_of_ways(7), 106_494);
        assert_eq!(Solution::num_of_ways(5000), 30_228_214);
    }
}
