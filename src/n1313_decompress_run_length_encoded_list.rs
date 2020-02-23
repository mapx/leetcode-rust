/**
 * [1313] Decompress Run-Length Encoded List
 *
 * We are given a list nums of integers representing a list compressed with run-length encoding.
 *
 * Consider each adjacent pair of elements [a, b] = [nums[2*i], nums[2*i+1]] (with i >= 0).  For each such pair, there are a elements with value b in the decompressed list.
 *
 * Return the decompressed list.
 *
 *  
 * Example 1:
 * Input: nums = [1,2,3,4]
 * Output: [2,4,4,4]
 *
 *  
 * Constraints:
 *
 *  2 <= nums.length <= 100
 *  nums.length % 2 == 0
 *  1 <= nums[i] <= 100
 *
 * Problem link: https://leetcode.com/problems/decompress-run-length-encoded-list/
 * Discuss link: https://leetcode.com/problems/decompress-run-length-encoded-list/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut r = vec![];
        let mut iter = nums.iter();
        loop {
            match iter.next() {
                Some(&a) => {
                    let b = iter.next().unwrap();
                    for _ in 0..(a as usize) {
                        r.push(*b);
                    }
                }
                None => return r,
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1313() {
        assert_eq!(
            Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
            vec![2, 4, 4, 4]
        );
    }
}
