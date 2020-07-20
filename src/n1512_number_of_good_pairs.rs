/**
 * [1512] Number of Good Pairs
 *
 * Given an array of integers `nums`.
 *
 * A pair `(i,j)` is called good if `nums[i]` == `nums[j]` and `i` < `j`.
 *
 * Return the number of good pairs.
 *
 *  
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1,1,3]
 * Output: 4
 * Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,1,1,1]
 * Output: 6
 * Explanation: Each pair in the array are good.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [1,2,3]
 * Output: 0
 *
 *
 *  
 * Constraints:
 *
 *
 *     `1 <= nums.length <= 100`
 *     `1 <= nums[i] <= 100`
 *
 * Problem link: https://leetcode.com/problems/number-of-good-pairs/
 * Discuss link: https://leetcode.com/problems/number-of-good-pairs/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counter = vec![0; 101];
        for i in nums {
            counter[i as usize] += 1;
        }
        let mut total = 0;
        for n in counter {
            total += (n - 1) * n / 2;
        }
        total
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1512() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
