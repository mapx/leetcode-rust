/**
 * [1344] Jump Game V
 *
 * Given an array of integers arr and an integer d. In one step you can jump from index i to index:
 *
 *
 *     i + x where: i + x < arr.length and  0 < x <= d.
 *     i - x where: i - x >= 0 and  0 < x <= d.
 *
 *
 * In addition, you can only jump from index i to index j if arr[i] > arr[j] and arr[i] > arr[k] for all indices k between i and j (More formally min(i, j) < k < max(i, j)).
 *
 * You can choose any index of the array and start jumping. Return the maximum number of indices you can visit.
 *
 * Notice that you can not jump outside of the array at any time.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/23/meta-chart.jpeg" style="width: 633px; height: 419px;" />
 *
 * Input: arr = [6,4,14,6,8,13,9,7,10,6,12], d = 2
 * Output: 4
 * Explanation: You can start at index 10. You can jump 10 --> 8 --> 6 --> 7 as shown.
 * Note that if you start at index 6 you can only jump to index 7. You cannot jump to index 5 because 13 > 9. You cannot jump to index 4 because index 5 is between index 4 and 6 and 13 > 9.
 * Similarly You cannot jump from index 3 to index 2 or index 1.
 *
 *
 * Example 2:
 *
 *
 * Input: arr = [3,3,3,3,3], d = 3
 * Output: 1
 * Explanation: You can start at any index. You always cannot jump to any index.
 *
 *
 * Example 3:
 *
 *
 * Input: arr = [7,6,5,4,3,2,1], d = 1
 * Output: 7
 * Explanation: Start at index 0. You can visit all the indicies.
 *
 *
 * Example 4:
 *
 *
 * Input: arr = [7,1,7,1,7,1], d = 2
 * Output: 2
 *
 *
 * Example 5:
 *
 *
 * Input: arr = [66], d = 1
 * Output: 1
 *
 *
 *  
 * Constraints:
 *
 *
 *     1 <= arr.length <= 1000
 *     1 <= arr[i] <= 10^5
 *     1 <= d <= arr.length
 *
 */
pub struct Solution {}

// submission codes start here

use std::cmp;

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut mem = vec![0; arr.len()];
        let mut indexed_arr: Vec<(usize, i32)> = arr.iter().cloned().enumerate().collect();
        indexed_arr.sort_by_key(|item| item.1);
        let mut max_result = 0;

        for (i, n) in indexed_arr {
            let mut local_best = 0;
            unsafe {
                for j in ((cmp::max(i as i32 - d, 0) as usize)..i).rev() {
                    if *arr.get_unchecked(j) >= n {
                        break;
                    }
                    local_best = cmp::max(local_best, *mem.get_unchecked(j));
                }
                for j in i + 1..cmp::min(i + d as usize + 1, arr.len()) {
                    if *arr.get_unchecked(j) >= n {
                        break;
                    }
                    local_best = cmp::max(local_best, *mem.get_unchecked(j));
                }
                local_best += 1;
                *mem.get_unchecked_mut(i) = local_best;
            }
            max_result = cmp::max(max_result, local_best);
        }

        max_result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1344() {
        assert_eq!(
            Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2),
            4
        );
        assert_eq!(Solution::max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
        assert_eq!(Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
        assert_eq!(Solution::max_jumps(vec![7, 1, 7, 1, 7, 1], 2), 2);
        assert_eq!(Solution::max_jumps(vec![66], 1), 1);
        assert_eq!(
            Solution::max_jumps(
                vec![
                    59, 8, 74, 27, 92, 36, 95, 78, 73, 54, 75, 37, 42, 15, 59, 84, 66, 25, 35, 61,
                    97, 16, 6, 52, 49, 18, 22, 70, 5, 59, 92, 85
                ],
                20
            ),
            8
        );
    }
}
