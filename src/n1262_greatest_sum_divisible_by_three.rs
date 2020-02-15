/**
 * [1262] Greatest Sum Divisible by Three
 *
 * Given an array nums of integers, we need to find the maximum possible sum of elements of the array such that it is divisible by three.
 * <ol>
 * </ol>
 *  
 * Example 1:
 *
 * Input: nums = [3,6,5,1,8]
 * Output: 18
 * Explanation: Pick numbers 3, 6, 1 and 8 their sum is 18 (maximum sum divisible by 3).
 * Example 2:
 *
 * Input: nums = [4]
 * Output: 0
 * Explanation: Since 4 is not divisible by 3, do not pick any number.
 *
 * Example 3:
 *
 * Input: nums = [1,2,3,4,4]
 * Output: 12
 * Explanation: Pick numbers 1, 3, 4 and 4 their sum is 12 (maximum sum divisible by 3).
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 4 * 10^4
 *     1 <= nums[i] <= 10^4
 *
 * Problem link: https://leetcode.com/problems/greatest-sum-divisible-by-three/
 * Discuss link: https://leetcode.com/problems/greatest-sum-divisible-by-three/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here
use std::cmp;
use std::i32;
use std::mem;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let smallest_two_elements = |a: &mut i32, b: &mut i32, new_element: i32| {
            if new_element <= *a {
                *b = *a;
                *a = new_element;
            } else if new_element <= *b {
                *b = new_element;
            }
        };

        let mut total = 0;
        let mut r1_a = i32::MAX;
        let mut r1_b = i32::MAX;
        let mut r2_a = i32::MAX;
        let mut r2_b = i32::MAX;

        for n in nums {
            total += n;
            match n % 3 {
                1 => smallest_two_elements(&mut r1_a, &mut r1_b, n),
                2 => smallest_two_elements(&mut r2_a, &mut r2_b, n),
                _ => (),
            }
        }

        let mut total1 = i32::MIN;
        let mut total2 = i32::MIN;
        match total % 3 {
            0 => {
                return total;
            }
            1 => {
                if r1_a < i32::MAX {
                    total1 = total - r1_a;
                }
                if r2_b < i32::MAX {
                    total2 = total - r2_a - r2_b;
                }
            }
            2 => {
                if r1_b < i32::MAX {
                    total1 = total - r1_a - r1_b;
                }
                if r2_a < i32::MAX {
                    total2 = total - r2_a;
                }
            }
            _ => unreachable!(),
        }
        cmp::max(total1, total2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1262() {
        assert_eq!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
        assert_eq!(Solution::max_sum_div_three(vec![4]), 0);
        assert_eq!(Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
    }
}
