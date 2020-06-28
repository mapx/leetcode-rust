/**
 * [1486] XOR Operation in an Array
 *
 * Given an integer `n` and an integer `start`.
 *
 * Define an array `nums` where `nums[i] = start + 2*i` (0-indexed) and `n == nums.length`.
 *
 * Return the bitwise XOR of all elements of `nums`.
 *
 *  
 * Example 1:
 *
 *
 * Input: n = 5, start = 0
 * Output: 8
 * Explanation: Array nums is equal to [0, 2, 4, 6, 8] where (0 ^ 2 ^ 4 ^ 6 ^ 8) = 8.
 * Where "^" corresponds to bitwise XOR operator.
 *
 *
 * Example 2:
 *
 *
 * Input: n = 4, start = 3
 * Output: 8
 * Explanation: Array nums is equal to [3, 5, 7, 9] where (3 ^ 5 ^ 7 ^ 9) = 8.
 *
 * Example 3:
 *
 *
 * Input: n = 1, start = 7
 * Output: 7
 *
 *
 * Example 4:
 *
 *
 * Input: n = 10, start = 5
 * Output: 2
 *
 *
 *  
 * Constraints:
 *
 *
 *     `1 <= n <= 1000`
 *     `0 <= start <= 1000`
 *     `n == nums.length`
 *
 * Problem link: https://leetcode.com/problems/xor-operation-in-an-array/
 * Discuss link: https://leetcode.com/problems/xor-operation-in-an-array/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut ret = 0;
        for i in 0..n {
            ret ^= (start + 2 * i);
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1486() {
        assert_eq!(Solution::xor_operation(5, 0), 8);
        assert_eq!(Solution::xor_operation(4, 3), 8);
        assert_eq!(Solution::xor_operation(1, 7), 7);
        assert_eq!(Solution::xor_operation(10, 5), 2);
    }
}
