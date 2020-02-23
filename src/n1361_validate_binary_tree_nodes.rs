use std::collections::HashSet;
use std::i32;

/**
 * [1361] Validate Binary Tree Nodes
 *
 * You have `n` binary tree nodes numbered from `0` to `n - 1` where node `i` has two children `leftChild[i]` and `rightChild[i]`, return `true` if and only if all the given nodes form exactly one valid binary tree.
 * If node `i` has no left child then `leftChild[i]` will equal `-1`, similarly for the right child.
 * Note that the nodes have no values and that we only use the node numbers in this problem.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex1.png" style="width: 195px; height: 287px;" />
 *
 * Input: n = 4, leftChild = [1,-1,3,-1], rightChild = [2,-1,-1,-1]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex2.png" style="width: 183px; height: 272px;" />
 *
 * Input: n = 4, leftChild = [1,-1,3,-1], rightChild = [2,3,-1,-1]
 * Output: false
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex3.png" style="width: 82px; height: 174px;" />
 *
 * Input: n = 2, leftChild = [1,0], rightChild = [-1,-1]
 * Output: false
 *
 * Example 4:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex4.png" style="width: 470px; height: 191px;" />
 *
 * Input: n = 6, leftChild = [1,-1,-1,4,-1,-1], rightChild = [2,-1,-1,5,-1,-1]
 * Output: false
 *
 *  
 * Constraints:
 *
 *     `1 <= n <= 10^4`
 *     `leftChild.length == rightChild.length == n`
 *     `-1 <= leftChild[i], rightChild[i] <= n - 1`
 *
 * Problem link: https://leetcode.com/problems/validate-binary-tree-nodes/
 * Discuss link: https://leetcode.com/problems/validate-binary-tree-nodes/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        seen.insert(0);
        fn travel(i: i32, left_child: &[i32], right_child: &[i32], seen: &mut HashSet<i32>) -> i32 {
            let mut total = 1i32;
            unsafe {
                for j in [
                    *left_child.get_unchecked(i as usize),
                    *right_child.get_unchecked(i as usize),
                ]
                .iter()
                {
                    if seen.contains(j) {
                        return i32::MIN;
                    }
                    if *j != -1 {
                        seen.insert(*j);
                        total += travel(*j, left_child, right_child, seen);
                    }
                }
            }
            total
        };

        n == travel(0, &left_child, &right_child, &mut seen)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1361() {
        assert_eq!(
            Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]),
            true
        );
        assert_eq!(
            Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, 3, -1, -1]),
            false
        );
        assert_eq!(
            Solution::validate_binary_tree_nodes(2, vec![1, 0], vec![-1, -1]),
            false
        );
        assert_eq!(
            Solution::validate_binary_tree_nodes(
                6,
                vec![1, -1, -1, 4, -1, -1],
                vec![2, -1, -1, 5, -1, -1],
            ),
            false
        );
    }
}
