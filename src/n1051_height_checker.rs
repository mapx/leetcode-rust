/**
 * [1051] Height Checker
 *
 * Students are asked to stand in non-decreasing order of heights for an annual photo.
 * Return the minimum number of students that must move in order for all students to be standing in non-decreasing order of height.
 *  
 *
 * Example 1:
 * Input: heights = [1,1,4,2,1,3]
 * Output: 3
 * Explanation:
 * Students with heights 4, 3 and the last 1 are not standing in the right positions.
 *
 *
 *  
 * Constraints:
 *
 *  1 <= heights.length <= 100
 *  1 <= heights[i] <= 100
 *
 * Problem link: https://leetcode.com/problems/height-checker/
 * Discuss link: https://leetcode.com/problems/height-checker/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted_heights = heights.clone();
        sorted_heights.sort_unstable();
        let mut cnts = 0;
        for (a, b) in heights.iter().zip(sorted_heights.iter()) {
            cnts += (a != b) as i32;
        }
        cnts
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1051() {
        assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
        assert_eq!(Solution::height_checker(vec![1]), 0);
        assert_eq!(Solution::height_checker(vec![1, 1]), 0);
        assert_eq!(Solution::height_checker(vec![1, 1, 2]), 0);
    }
}
