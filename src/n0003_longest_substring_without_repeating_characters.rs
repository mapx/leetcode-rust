/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">"abcabcbb"</span>
 * Output: <span id="example-output-1">3
 * Explanation:</span> The answer is "abc", with the length of 3.
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">"bbbbb"</span>
 * Output: <span id="example-output-2">1
 * </span><span id="example-output-1">Explanation: T</span>he answer is "b", with the length of 1.
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: <span id="example-input-3-1">"pwwkew"</span>
 * Output: <span id="example-output-3">3
 * </span><span id="example-output-1">Explanation: </span>The answer is "wke", with the length of 3.
 *              Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 * </div>
 * </div>
 * </div>
 *
 * Problem link: https://leetcode.com/problems/longest-substring-without-repeating-characters/
 * Discuss link: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

use std::cmp;
use std::collections::HashSet;

impl Solution {
    fn length_of_longest_substring_with_set(s: String) -> i32 {
        let mut lo = -1;
        let mut mem = HashSet::new();
        let mut best = 0;
        let bytes = s.as_bytes();
        for (hi, c) in bytes.iter().clone().enumerate() {
            while mem.contains(&c) {
                lo += 1;
                mem.remove(&bytes[lo as usize]);
            }
            mem.insert(c);
            best = cmp::max(best, (hi as i32) - lo);
        }
        best
    }

    // bitwise operator version, fastest
    pub fn length_of_longest_substring(s: String) -> i32 {
        // return Self::length_of_longest_substring_with_set(s);
        let mut lo = -1;
        let mut mem = 0u128;
        let mut best = 0;
        let bytes = s.as_bytes();
        for (hi, &c) in bytes.iter().enumerate() {
            while (mem & (1u128 << (c as u128))) != 0 {
                lo += 1;
                mem &= !(1u128 << (bytes[lo as usize] as u128));
            }
            mem |= 1u128 << (c as u128);
            best = cmp::max(best, (hi as i32) - lo);
        }
        best
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
