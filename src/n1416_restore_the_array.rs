/**
 * [1416] Restore The Array
 *
 * A program was supposed to print an array of integers. The program forgot to print whitespaces and the array is printed as a string of digits and all we know is that all integers in the array were in the range `[1, k]` and there are no leading zeros in the array.
 *
 * Given the string `s` and the integer `k`. There can be multiple ways to restore the array.
 *
 * Return the number of possible array that can be printed as a string `s` using the mentioned program.
 *
 * The number of ways could be very large so return it modulo `10^9 + 7`
 *
 *  
 * Example 1:
 *
 *
 * Input: s = "1000", k = 10000
 * Output: 1
 * Explanation: The only possible array is [1000]
 *
 *
 * Example 2:
 *
 *
 * Input: s = "1000", k = 10
 * Output: 0
 * Explanation: There cannot be an array that was printed this way and has all integer >= 1 and <= 10.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "1317", k = 2000
 * Output: 8
 * Explanation: Possible arrays are [1317],[131,7],[13,17],[1,317],[13,1,7],[1,31,7],[1,3,17],[1,3,1,7]
 *
 *
 * Example 4:
 *
 *
 * Input: s = "2020", k = 30
 * Output: 1
 * Explanation: The only possible array is [20,20]. [2020] is invalid because 2020 > 30. [2,020] is ivalid because 020 contains leading zeros.
 *
 *
 * Example 5:
 *
 *
 * Input: s = "1234567890", k = 90
 * Output: 34
 *
 *
 *  
 * Constraints:
 *
 *
 *     `1 <= s.length <= 10^5`.
 *     `s` consists of only digits and doesn't contain leading zeros.
 *     `1 <= k <= 10^9`.
 *
 * Problem link: https://leetcode.com/problems/restore-the-array/
 * Discuss link: https://leetcode.com/problems/restore-the-array/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here
use std::str::FromStr;

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let modulo = 1_000_000_007;
        let mut mem = vec![modulo + 1; s.len() + 1];
        mem[s.len()] = 1; // mark the initial status
        let mut stack = vec![0usize];
        while let Some(&i) = stack.last() {
            unsafe {
                if *mem.get_unchecked(i) < modulo {
                    stack.pop();
                    continue;
                }
            }
            let mut cnt = 0;
            let mut solved = true;
            /// Are all sub-problems solved?
            for j in (i + 1)..(s.len() + 1) {
                if i32::from_str(&s[i..j]).unwrap_or(k + 1) > k {
                    break;
                }
                unsafe {
                    if *s_bytes.get_unchecked(j) != b'0' {
                        let mem_j = *mem.get_unchecked(j);
                        if mem_j >= modulo {
                            solved = false;
                            stack.push(j);
                        } else {
                            cnt = (cnt + mem_j) % modulo;
                        }
                    }
                }
            }
            if solved {
                unsafe {
                    *mem.get_unchecked_mut(i) = cnt;
                }
            }
        }
        mem[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1416() {
        assert_eq!(Solution::number_of_arrays("1000".to_string(), 10000), 1);
        assert_eq!(Solution::number_of_arrays("1000".to_string(), 10), 0);
        assert_eq!(Solution::number_of_arrays("1317".to_string(), 2000), 8);
        assert_eq!(Solution::number_of_arrays("2020".to_string(), 30), 1);
        assert_eq!(Solution::number_of_arrays("1234567890".to_string(), 90), 34);
        assert_eq!(Solution::number_of_arrays("13310528612424986641739914122916630296312311204961192234215510514712812017722214344326519532631151261197962272322417010925931030322817778689516214827211124271263171136124723411031115319311132108310".to_string(), 331), 354_614_528
        );
    }
}
