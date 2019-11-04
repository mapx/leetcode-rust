/**
 * [1371] Minimum Remove to Make Valid Parentheses
 *
 * Given a string <font face="monospace">s</font> of '(' , ')' and lowercase English characters.
 *
 * Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.
 *
 * Formally, a parentheses string is valid if and only if:
 *
 *
 * 	It is the empty string, contains only lowercase characters, or
 * 	It can be written as AB (A concatenated with B), where A and B are valid strings, or
 * 	It can be written as (A), where A is a valid string.
 *
 *
 *  
 * Example 1:
 *
 *
 * Input: s = "lee(t(c)o)de)"
 * Output: "lee(t(c)o)de"
 * Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "a)b(c)d"
 * Output: "ab(c)d"
 *
 *
 * Example 3:
 *
 *
 * Input: s = "))(("
 * Output: ""
 * Explanation: An empty string is also valid.
 *
 *
 * Example 4:
 *
 *
 * Input: s = "(a(b(c)d)"
 * Output: "a(b(c)d)"
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is one of  '(' , ')' and lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut arr = vec![];
        let mut brackets = vec![];
        for (i, mut c) in s.as_bytes().iter().cloned().enumerate() {
            match c {
                b'(' => {
                    brackets.push(i as usize);
                }
                b')' => {
                    if brackets.is_empty() {
                        c = 0;
                    } else {
                        brackets.pop();
                    }
                }
                _ => {}
            }
            arr.push(c);
        }
        for &i in &brackets {
            arr[i] = 0;
        }
        arr.retain(|&c| c > 0);
        String::from_utf8(arr).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1371() {
        assert_eq!(
            Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
            "lee(t(c)o)de".to_string()
        );
        assert_eq!(
            Solution::min_remove_to_make_valid("a)b(c)d".to_string()),
            "ab(c)d".to_string()
        );
        assert_eq!(
            Solution::min_remove_to_make_valid("))((".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::min_remove_to_make_valid("(a(b(c)d)".to_string()),
            "a(b(c)d)".to_string()
        );
    }
}
