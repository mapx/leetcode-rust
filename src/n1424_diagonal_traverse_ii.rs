/**
 * [1424] Diagonal Traverse II
 *
 * Given a list of lists of integers, `nums`, return all elements of `nums` in diagonal order as shown in the below images.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_1_1784.png" style="width: 158px; height: 143px;" />
 *
 * Input: nums = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,4,2,7,5,3,8,6,9]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_2_1784.png" style="width: 230px; height: 177px;" />
 *
 * Input: nums = [[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]]
 * Output: [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]
 *
 * Example 3:
 *
 * Input: nums = [[1,2,3],[4],[5,6,7],[8],[9,10,11]]
 * Output: [1,4,2,5,3,8,6,9,7,10,11]
 *
 * Example 4:
 *
 * Input: nums = [[1,2,3,4,5,6]]
 * Output: [1,2,3,4,5,6]
 *
 *  
 * Constraints:
 *
 *     `1 <= nums.length <= 10^5`
 *     `1 <= nums[i].length <= 10^5`
 *     `1 <= nums[i][j] <= 10^9`
 *     There at most `10^5` elements in `nums`.
 *
 * Problem link: https://leetcode.com/problems/diagonal-traverse-ii/
 * Discuss link: https://leetcode.com/problems/diagonal-traverse-ii/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![];
        let mut q1 = vec![];
        let mut q2 = vec![];
        for row in nums {
            q1.push(row.into_iter());
            while let Some(mut it) = q1.pop() {
                if let Some(n) = it.next() {
                    ret.push(n);
                    q2.push(it);
                }
            }
            q2.reverse();
            q1.append(&mut q2);
        }
        q1.reverse();
        while !q1.is_empty() {
            for mut it in q1.drain(..) {
                if let Some(n) = it.next() {
                    ret.push(n);
                    q2.push(it);
                }
            }
            q1.append(&mut q2);
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1424() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 4, 2, 7, 5, 3, 8, 6, 9]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7],
                vec![8],
                vec![9, 10, 11],
                vec![12, 13, 14, 15, 16]
            ]),
            vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![
                vec![1, 2, 3],
                vec![4],
                vec![5, 6, 7],
                vec![8],
                vec![9, 10, 11]
            ]),
            vec![1, 4, 2, 5, 3, 8, 6, 9, 7, 10, 11]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2, 3, 4, 5, 6]]),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
}
