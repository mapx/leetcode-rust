/**
 * [1356] Sort Integers by The Number of 1 Bits
 *
 * Given an integer array `arr`. You have to sort the integers in the array in ascending order by the number of 1's in their binary representation and in case of two or more integers have the same number of 1's you have to sort them in ascending order.
 * Return the sorted array.
 *  
 * Example 1:
 *
 * Input: arr = [0,1,2,3,4,5,6,7,8]
 * Output: [0,1,2,4,8,3,5,6,7]
 * Explantion: [0] is the only integer with 0 bits.
 * [1,2,4,8] all have 1 bit.
 * [3,5,6] have 2 bits.
 * [7] has 3 bits.
 * The sorted array by bits is [0,1,2,4,8,3,5,6,7]
 *
 * Example 2:
 *
 * Input: arr = [1024,512,256,128,64,32,16,8,4,2,1]
 * Output: [1,2,4,8,16,32,64,128,256,512,1024]
 * Explantion: All integers have 1 bit in the binary representation, you should just sort them in ascending order.
 *
 * Example 3:
 *
 * Input: arr = [10000,10000]
 * Output: [10000,10000]
 *
 * Example 4:
 *
 * Input: arr = [2,3,5,7,11,13,17,19]
 * Output: [2,3,5,17,7,11,13,19]
 *
 * Example 5:
 *
 * Input: arr = [10,100,1000,10000]
 * Output: [10,100,10000,1000]
 *
 *  
 * Constraints:
 *
 *     `1 <= arr.length <= 500`
 *     `0 <= arr[i] <= 10^4`
 *
 * Problem link: https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/
 * Discuss link: https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        arr.sort_by_key(|&n| {
            let mut count = 0;
            let mut num = n;
            while num > 0 {
                count += (num & 1) as usize;
                num >>= 1;
            }
            (count, n)
        });
        arr
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1356() {
        assert_eq!(
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![10000, 10000]),
            vec![10000, 10000]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![2, 3, 5, 7, 11, 13, 17, 19]),
            vec![2, 3, 5, 17, 7, 11, 13, 19]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![10, 100, 1000, 10000]),
            vec![10, 100, 10000, 1000]
        );
    }
}
