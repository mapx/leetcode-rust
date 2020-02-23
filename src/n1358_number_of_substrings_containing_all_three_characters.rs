/**
 * [1358] Number of Substrings Containing All Three Characters
 *
 * Given a string `s` consisting only of characters a, b and c.
 *
 * Return the number of substrings containing at least one occurrence of all these characters a, b and c.
 *
 *  
 * Example 1:
 *
 *
 * Input: s = "abcabc"
 * Output: 10
 * Explanation: The substrings containing at least one occurrence of the characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again).
 *
 *
 * Example 2:
 *
 *
 * Input: s = "aaacb"
 * Output: 3
 * Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb".
 *
 *
 * Example 3:
 *
 *
 * Input: s = "abc"
 * Output: 1
 *
 *
 *  
 * Constraints:
 *
 *
 *     `3 <= s.length <= 5 x 10^4`
 *     `s` only consists of a, b or c characters.
 *
 * Problem link: https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
 * Discuss link: https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut seen = vec![0; 3];
        let mut total = 0;
        let mut lo = 0;
        let bytes = s.into_bytes();
        for (hi, c) in bytes.iter().enumerate() {
            unsafe {
                *seen.get_unchecked_mut((c - b'a') as usize) += 1;
                while seen.iter().all(|&n| n > 0) {
                    total += bytes.len() - hi; // counts the number of strings that have bytes[lo..hi] as its prefix
                    *seen.get_unchecked_mut((*bytes.get_unchecked(lo) - b'a') as usize) -= 1;
                    lo += 1;
                }
            }
        }
        total as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1358() {
        assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10);
        assert_eq!(Solution::number_of_substrings("aaacb".to_string()), 3);
        assert_eq!(Solution::number_of_substrings("abc".to_string()), 1);
    }
}
