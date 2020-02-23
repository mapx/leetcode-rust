/**
 * [1310] XOR Queries of a Subarray
 *
 * Given the array arr of positive integers and the array queries where queries[i] = [Li, Ri], for each query i compute the XOR of elements from Li to Ri (that is, arr[Li] xor arr[Li+1] xor ... xor arr[Ri] ). Return an array containing the result for the given queries.
 *  
 * Example 1:
 *
 *
 * Input: arr = [1,3,4,8], queries = [[0,1],[1,2],[0,3],[3,3]]
 * Output: [2,7,14,8]
 * Explanation:
 * The binary representation of the elements in the array are:
 * 1 = 0001
 * 3 = 0011
 * 4 = 0100
 * 8 = 1000
 * The XOR values for queries are:
 * [0,1] = 1 xor 3 = 2
 * [1,2] = 3 xor 4 = 7
 * [0,3] = 1 xor 3 xor 4 xor 8 = 14
 * [3,3] = 8
 *
 *
 * Example 2:
 *
 *
 * Input: arr = [4,8,2,10], queries = [[2,3],[1,3],[0,0],[0,3]]
 * Output: [8,0,4,4]
 *
 *
 *  
 * Constraints:
 *
 *
 *  1 <= arr.length <= 3 * 10^4
 *  1 <= arr[i] <= 10^9
 *  1 <= queries.length <= 3 * 10^4
 *  queries[i].length == 2
 *  0 <= queries[i][0] <= queries[i][1] < arr.length
 *
 * Problem link: https://leetcode.com/problems/xor-queries-of-a-subarray/
 * Discuss link: https://leetcode.com/problems/xor-queries-of-a-subarray/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut last = 0;
        let xor_acc: Vec<i32> = vec![0]
            .iter()
            .chain(arr.iter())
            .map(|n| {
                last ^= *n;
                last
            })
            .collect();
        queries
            .iter()
            .map(|query| xor_acc[query[0] as usize] ^ xor_acc[query[1] as usize + 1])
            .collect::<Vec<_>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1310() {
        assert_eq!(
            Solution::xor_queries(
                vec![1, 3, 4, 8],
                vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
            ),
            vec![2, 7, 14, 8]
        );
        assert_eq!(
            Solution::xor_queries(
                vec![4, 8, 2, 10],
                vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]]
            ),
            vec![8, 0, 4, 4]
        );
    }
}
