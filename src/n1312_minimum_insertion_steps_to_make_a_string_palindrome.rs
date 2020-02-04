/**
 * [1312] Minimum Insertion Steps to Make a String Palindrome
 *
 * Given a string s. In one step you can insert any character at any index of the string.
 *
 * Return the minimum number of steps to make s palindrome.
 *
 * A Palindrome String is one that reads the same backward as well as forward.
 *
 *  
 * Example 1:
 *
 *
 * Input: s = "zzazz"
 * Output: 0
 * Explanation: The string "zzazz" is already palindrome we don't need any insertions.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "mbadm"
 * Output: 2
 * Explanation: String can be "mbdadbm" or "mdbabdm".
 *
 *
 * Example 3:
 *
 *
 * Input: s = "leetcode"
 * Output: 5
 * Explanation: Inserting 5 characters the string becomes "leetcodocteel".
 *
 *
 * Example 4:
 *
 *
 * Input: s = "g"
 * Output: 0
 *
 *
 * Example 5:
 *
 *
 * Input: s = "no"
 * Output: 1
 *
 *
 *  
 * Constraints:
 *
 *
 *     1 <= s.length <= 500
 *     All characters of s are lower case English letters.
 *
 */
pub struct Solution {}

// submission codes start here

use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        if s.len() <= 1 {
            return 0;
        }

        #[derive(Hash, Eq, PartialEq)]
        struct PalindromePair<'a> {
            l: &'a str,
            r: &'a str,
        }
        let mut mem = HashMap::new();

        fn min_diff<'a>(
            l: &'a str,
            r: &'a str,
            mem: &mut HashMap<PalindromePair<'a>, usize>,
        ) -> usize {
            let k = PalindromePair { l, r };
            if let Some(steps) = mem.get(&k) {
                return *steps;
            }
            let mut i = 0;
            let min_len = cmp::min(l.len(), r.len());
            for (&c0, &c1) in l.as_bytes().iter().zip(r.as_bytes()) {
                if c0 != c1 {
                    break;
                }
                i += 1;
            }

            let steps = if i >= min_len {
                cmp::max(l.len(), r.len()) - i
            } else {
                1 + cmp::min(
                    min_diff(&l[i + 1..], &r[i..], mem),
                    min_diff(&l[i..], &r[i + 1..], mem),
                )
            };
            mem.insert(k, steps);
            steps
        }

        let mut steps = s.len();
        let s_rev: String = s.chars().rev().collect();
        for i in 1..s.len() {
            steps = cmp::min(steps, min_diff(&s_rev[s.len() - i..], &s[i..], &mut mem));
            steps = cmp::min(
                steps,
                min_diff(&s_rev[s.len() - i - 1..], &s[i..], &mut mem),
            );
        }
        steps as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1312() {
        assert_eq!(Solution::min_insertions("zzazz".to_string()), 0);
        assert_eq!(Solution::min_insertions("mbadm".to_string()), 2);
        assert_eq!(Solution::min_insertions("leetcode".to_string()), 5);
        assert_eq!(Solution::min_insertions("g".to_string()), 0);
        assert_eq!(Solution::min_insertions("no".to_string()), 1);
    }
}
