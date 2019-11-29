/**
 * [1395] Minimum Time Visiting All Points
 *
 * On a plane there are n points with integer coordinates points[i] = [xi, yi]. Your task is to find the minimum time in seconds to visit all points.
 * You can move according to the next rules:
 *
 * 	In one second always you can either move vertically, horizontally by one unit or diagonally (it means to move one unit vertically and one unit horizontally in one second).
 * 	You have to visit the points in the same order as they appear in the array.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/14/1626_example_1.PNG" style="width: 500px; height: 428px;" />
 * Input: points = [[1,1],[3,4],[-1,0]]
 * Output: 7
 * Explanation: One optimal path is [1,1] -> [2,2] -> [3,3] -> [3,4] -> [2,3] -> [1,2] -> [0,1] -> [-1,0]   
 * Time from [1,1] to [3,4] = 3 seconds
 * Time from [3,4] to [-1,0] = 4 seconds
 * Total time = 7 seconds
 * Example 2:
 *
 * Input: points = [[3,2],[-2,2]]
 * Output: 5
 *
 *  
 * Constraints:
 *
 * 	points.length == n
 * 	1 <= n <= 100
 * 	points[i].length == 2
 * 	-1000 <= points[i][0], points[i][1] <= 1000
 *
 */
pub struct Solution {}

// submission codes start here
use std::cmp;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut t = 0;
        if let Some(p) = points.first() {
            let mut px = p[0];
            let mut py = p[1];
            for nxt_p in points {
                let x = nxt_p[0];
                let y = nxt_p[1];
                let dx = (x - px).abs();
                let dy = (y - py).abs();
                let diag = cmp::min(dx, dy);
                t += dx + dy - diag;
                px = x;
                py = y;
            }
        }
        t
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1395() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        );
        assert_eq!(Solution::min_time_to_visit_all_points(vec![vec![3, 2]]), 0);
    }
}
