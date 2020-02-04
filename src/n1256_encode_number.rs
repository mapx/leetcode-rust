/**
 * [1256] Encode Number
 *
 * Given a non-negative integer num, Return its encoding string.
 *
 * The encoding is done by converting the integer to a string using a secret function that you should deduce from the following table:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/06/21/encode_number.png" style="width: 164px; height: 360px;" />
 *
 *  
 * Example 1:
 *
 *
 * Input: num = 23
 * Output: "1000"
 *
 *
 * Example 2:
 *
 *
 * Input: num = 107
 * Output: "101100"
 *
 *
 *  
 * Constraints:
 *
 *
 *     0 <= num <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /**
     * Assume g(n) = "1" + f(n)
     * we can find:
     * g(0) = "1" g(1) = "10" g(2) = "111" g(3) = "100" g(4) = "101" g(5) = "110" g(6) = "111"
     *
     * Now everything is obvious:
     * g(n) = binary(n + 1)
     * "1" + f(n) = binary(n + 1)
     * f(n) = binary(n + 1).substring(1)
     */
    pub fn encode(num: i32) -> String {
        format!("{:b}", num + 1)[1..].to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1256() {
        assert_eq!(Solution::encode(23), "1000");
        assert_eq!(Solution::encode(107), "101100");
    }
}
