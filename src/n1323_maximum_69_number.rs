/**
 * [1323] Maximum 69 Number
 *
 * Given a positive integer num consisting only of digits 6 and 9.
 *
 * Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).
 *
 *  
 * Example 1:
 *
 *
 * Input: num = 9669
 * Output: 9969
 * Explanation:
 * Changing the first digit results in 6669.
 * Changing the second digit results in 9969.
 * Changing the third digit results in 9699.
 * Changing the fourth digit results in 9666.
 * The maximum number is 9969.
 *
 *
 * Example 2:
 *
 *
 * Input: num = 9996
 * Output: 9999
 * Explanation: Changing the last digit 6 to 9 results in the maximum number.
 *
 * Example 3:
 *
 *
 * Input: num = 9999
 * Output: 9999
 * Explanation: It is better not to apply any change.
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= num <= 10^4
 * 	num's digits are 6 or 9.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut power = 10;
        while power < num {
            power *= 10;
        }

        let mut n = num;
        let mut r = 0;
        let mut not_changed = true;
        while n > 0 {
            power /= 10;
            let mut c = n / power;
            n -= c * power;
            if not_changed && c == 6 {
                c = 9;
                not_changed = false;
            }
            r = r * 10 + c;
        }
        r as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1323() {
        assert_eq!(Solution::maximum69_number(9669), 9969);
        assert_eq!(Solution::maximum69_number(9996), 9999);
        assert_eq!(Solution::maximum69_number(9999), 9999);
    }
}
