/**
 * [7] Reverse Integer
 *
 * Given a 32-bit signed integer, reverse digits of an integer.
 *
 * Example 1:
 *
 *
 * Input: 123
 * Output: 321
 *
 *
 * Example 2:
 *
 *
 * Input: -123
 * Output: -321
 *
 *
 * Example 3:
 *
 *
 * Input: 120
 * Output: 21
 *
 *
 * Note:
 * Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
 *
 * Problem link: https://leetcode.com/problems/reverse-integer/
 * Discuss link: https://leetcode.com/problems/reverse-integer/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x as i64;
        let mut result = 0;
        while x != 0 {
            result = result * 10 + x % 10;
            x /= 10;
        }
        if -(2i64.pow(31)) <= result && result < 2i64.pow(31) {
            result as i32
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-123_000), -321);
        assert_eq!(Solution::reverse((2i64.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse((-(2i64.pow(31))) as i32), 0);
        assert_eq!(Solution::reverse(1_534_236_469), 0);
        assert_eq!(Solution::reverse(32768), 86723);
    }
}
