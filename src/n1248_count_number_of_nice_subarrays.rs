/**
 * [1248] Count Number of Nice Subarrays
 *
 * Given an array of integers nums and an integer k. A subarray is called nice if there are k odd numbers on it.
 * Return the number of nice sub-arrays.
 *  
 * Example 1:
 *
 * Input: nums = [1,1,2,1,1], k = 3
 * Output: 2
 * Explanation: The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].
 *
 * Example 2:
 *
 * Input: nums = [2,4,6], k = 1
 * Output: 0
 * Explanation: There is no odd numbers in the array.
 *
 * Example 3:
 *
 * Input: nums = [2,2,2,1,2,2,1,2,2,2], k = 2
 * Output: 16
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 50000
 *     1 <= nums[i] <= 10^5
 *     1 <= k <= nums.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    /**
     * Each nice subarry consists of a k-odds subarry core, a left neighbour part of the core, and the
     * right neighbour part of it. So each nice subbary has (lo_part * hi_part) variants.
     */
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut odds = vec![];
        for (i, &n) in nums.iter().enumerate() {
            if n % 2 != 0 {
                odds.push(i as i32);
            }
        }
        if k > odds.len() {
            return 0;
        }
        odds.push(nums.len() as i32);
        let mut cnts = 0;
        let mut last = -1;
        for i in 0..(odds.len() - k) {
            let lo_part = odds[i] - last;
            last = odds[i];
            let hi_part = odds[i + k] - odds[i + k - 1];
            cnts += hi_part * lo_part;
        }
        cnts
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1248() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
        assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
        assert_eq!(
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }
}
