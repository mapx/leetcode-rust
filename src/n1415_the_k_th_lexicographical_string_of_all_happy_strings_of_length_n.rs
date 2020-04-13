/**
 * [1415] The k-th Lexicographical String of All Happy Strings of Length n
 *
 * A happy string is a string that:
 *
 *
 *     consists only of letters of the set `['a', 'b', 'c']`.
 *     `s[i] != s[i + 1]` for all values of `i` from `1` to `s.length - 1` (string is 1-indexed).
 *
 *
 * For example, strings "abc", "ac", "b" and "abcbabcbcb" are all happy strings and strings "aa", "baa" and "ababbc" are not happy strings.
 *
 * Given two integers `n` and `k`, consider a list of all happy strings of length `n` sorted in lexicographical order.
 *
 * Return the kth string of this list or return an empty string if there are less than `k` happy strings of length `n`.
 *
 *  
 * Example 1:
 *
 *
 * Input: n = 1, k = 3
 * Output: "c"
 * Explanation: The list ["a", "b", "c"] contains all happy strings of length 1. The third string is "c".
 *
 *
 * Example 2:
 *
 *
 * Input: n = 1, k = 4
 * Output: ""
 * Explanation: There are only 3 happy strings of length 1.
 *
 *
 * Example 3:
 *
 *
 * Input: n = 3, k = 9
 * Output: "cab"
 * Explanation: There are 12 different happy string of length 3 ["aba", "abc", "aca", "acb", "bab", "bac", "bca", "bcb", "cab", "cac", "cba", "cbc"]. You will find the 9th string = "cab"
 *
 *
 * Example 4:
 *
 *
 * Input: n = 2, k = 7
 * Output: ""
 *
 *
 * Example 5:
 *
 *
 * Input: n = 10, k = 100
 * Output: "abacbabacb"
 *
 *
 *  
 * Constraints:
 *
 *
 *     `1 <= n <= 10`
 *     `1 <= k <= 100`
 *
 *
 * <div id="vidyowebrtcscreenshare_is_installed"> </div>
 * Problem link: https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
 * Discuss link: https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as u32;
        let mut sub = 2usize.pow(n - 1);
        let mut k = (k - 1) as usize;
        if 3 * sub <= k {
            return "".to_string();
        }
        let mut last = [b'a', b'b', b'c'][k / sub];
        let mut ret = vec![last];
        let nxt = [[b'b', b'c'], [b'a', b'c'], [b'a', b'b']];
        for _ in 1..n {
            k %= sub;
            sub /= 2;
            unsafe {
                last = *nxt
                    .get_unchecked((last - b'a') as usize)
                    .get_unchecked(k / sub);
            }
            ret.push(last);
        }
        unsafe { String::from_utf8_unchecked(ret) }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1415() {
        assert_eq!(Solution::get_happy_string(1, 3), "c".to_string());
        assert_eq!(Solution::get_happy_string(1, 4), "".to_string());
        assert_eq!(Solution::get_happy_string(3, 9), "cab".to_string());
        assert_eq!(Solution::get_happy_string(2, 7), "".to_string());
        assert_eq!(
            Solution::get_happy_string(10, 100),
            "abacbabacb".to_string()
        );
    }
}
