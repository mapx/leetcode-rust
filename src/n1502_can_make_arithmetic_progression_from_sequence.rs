/**
 * [1502] Can Make Arithmetic Progression From Sequence
 *
 * Given an array of numbers `arr`. A sequence of numbers is called an arithmetic progression if the difference between any two consecutive elements is the same.
 * Return `true` if the array can be rearranged to form an arithmetic progression, otherwise, return `false`.
 *  
 * Example 1:
 *
 * Input: arr = [3,5,1]
 * Output: true
 * Explanation: We can reorder the elements as [1,3,5] or [5,3,1] with differences 2 and -2 respectively, between each consecutive elements.
 *
 * Example 2:
 *
 * Input: arr = [1,2,4]
 * Output: false
 * Explanation: There is no way to reorder the elements to obtain an arithmetic progression.
 *
 *  
 * Constraints:
 *
 *     `2 <= arr.length <= 1000`
 *     `-10^6 <= arr[i] <= 10^6`
 *
 * Problem link: https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/
 * Discuss link: https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort();
        if arr.len() < 2 {
            return false;
        }
        let mut last;
        let mut diff;
        unsafe {
            let first = *arr.get_unchecked(0);
            diff = *arr.get_unchecked(1) - first;
            last = first - diff;
        }
        for n in arr {
            if n - last != diff {
                return false;
            }
            last = n;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1502() {
        assert_eq!(
            Solution::can_make_arithmetic_progression(vec![3, 5, 1]),
            true
        );
        assert_eq!(
            Solution::can_make_arithmetic_progression(vec![1, 2, 4]),
            false
        );
    }
}
