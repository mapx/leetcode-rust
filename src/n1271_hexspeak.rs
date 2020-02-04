/**
 * [1271] Hexspeak
 *
 * A decimal number can be converted to its Hexspeak representation by first converting it to an uppercase hexadecimal string, then replacing all occurrences of the digit 0 with the letter O, and the digit 1 with the letter I.  Such a representation is valid if and only if it consists only of the letters in the set {"A", "B", "C", "D", "E", "F", "I", "O"}.
 * Given a string num representing a decimal integer N, return the Hexspeak representation of N if it is valid, otherwise return "ERROR".
 *  
 * Example 1:
 *
 * Input: num = "257"
 * Output: "IOI"
 * Explanation:  257 is 101 in hexadecimal.
 *
 * Example 2:
 *
 * Input: num = "3"
 * Output: "ERROR"
 *
 *  
 * Constraints:
 *
 *     1 <= N <= 10^12
 *     There are no leading zeros in the given string.
 *     All answers must be in uppercase letters.
 *
 */
pub struct Solution {}

// submission codes start here
use std::str;
impl Solution {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_hexspeak(num: String) -> String {
        let hex_str = match num.parse::<usize>() {
            Ok(n) => format!("{:X}", n),
            _ => return "ERROR".to_string(),
        };
        let mut buf = vec![];
        for c in hex_str.as_bytes() {
            match c {
                b'A' | b'B' | b'C' | b'D' | b'E' | b'F' => buf.push(*c),
                b'1' => buf.push(b'I'),
                b'0' => buf.push(b'O'),
                _ => return "ERROR".to_string(),
            }
        }
        unsafe { str::from_utf8_unchecked(&buf[..]).to_string() }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1271() {
        assert_eq!(Solution::to_hexspeak("257".to_string()), "IOI".to_string());
        assert_eq!(Solution::to_hexspeak("3".to_string()), "ERROR".to_string());
        assert_eq!(
            Solution::to_hexspeak("747823223228".to_string()),
            "AEIDBCDIBC".to_string()
        );
    }
}
