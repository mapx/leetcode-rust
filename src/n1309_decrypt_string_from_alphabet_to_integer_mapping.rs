/**
 * [1309] Decrypt String from Alphabet to Integer Mapping
 *
 * Given a string s formed by digits ('0' - '9') and '#' . We want to map s to English lowercase characters as follows:
 *
 *
 *     Characters ('a' to 'i') are represented by ('1' to '9') respectively.
 *     Characters ('j' to 'z') are represented by ('10#' to '26#') respectively.
 *
 *
 * Return the string formed after mapping.
 *
 * It's guaranteed that a unique mapping will always exist.
 *
 *  
 * Example 1:
 *
 *
 * Input: s = "10#11#12"
 * Output: "jkab"
 * Explanation: "j" -> "10#" , "k" -> "11#" , "a" -> "1" , "b" -> "2".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "1326#"
 * Output: "acz"
 *
 *
 * Example 3:
 *
 *
 * Input: s = "25#"
 * Output: "y"
 *
 *
 * Example 4:
 *
 *
 * Input: s = "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#"
 * Output: "abcdefghijklmnopqrstuvwxyz"
 *
 *
 *  
 * Constraints:
 *
 *
 *     1 <= s.length <= 1000
 *     s[i] only contains digits letters ('0'-'9') and '#' letter.
 *     s will be valid string such that mapping is always possible.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut i = 0;
        let bytes = s.as_bytes();
        let mut arr = vec![];
        let base = b'a' - 1;
        while i < bytes.len() {
            if i + 2 < bytes.len() && bytes[i + 2] == b'#' {
                arr.push(base + (bytes[i] - b'0') * 10 + bytes[i + 1] - b'0');
                i += 3;
            } else {
                arr.push(base + bytes[i] - b'0');
                i += 1;
            }
        }
        unsafe { String::from_utf8_unchecked(arr) }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1309() {
        assert_eq!(
            Solution::freq_alphabets("10#11#12".to_string()),
            "jkab".to_string()
        );
        assert_eq!(
            Solution::freq_alphabets("1326#".to_string()),
            "acz".to_string()
        );
        assert_eq!(Solution::freq_alphabets("25#".to_string()), "y".to_string());
        assert_eq!(
            Solution::freq_alphabets(
                "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string()
            ),
            "abcdefghijklmnopqrstuvwxyz".to_string()
        );
    }
}
