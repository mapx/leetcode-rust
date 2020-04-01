/**
 * [1367] Linked List in Binary Tree
 *
 * Given a binary tree `root` and a linked list with `head` as the first node.
 *
 * Return True if all the elements in the linked list starting from the `head` correspond to some downward path connected in the binary tree otherwise return False.
 *
 * In this context downward path means a path that starts at some node and goes downwards.
 *
 *  
 * Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/12/sample_1_1720.png" style="width: 220px; height: 280px;" />
 *
 *
 * Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
 * Output: true
 * Explanation: Nodes in blue form a subpath in the binary Tree.  
 *
 *
 * Example 2:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/12/sample_2_1720.png" style="width: 220px; height: 280px;" />
 *
 *
 * Input: head = [1,4,2,6], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: head = [1,4,2,6,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
 * Output: false
 * Explanation: There is no path in the binary tree that contains all the elements of the linked list from `head`.
 *
 *
 *  
 * Constraints:
 *
 *
 *     `1 <= node.val <= 100` for each node in the linked list and binary tree.
 *     The given linked list will contain between `1` and `100` nodes.
 *     The given binary tree will contain between `1` and `2500` nodes.
 *
 * Problem link: https://leetcode.com/problems/linked-list-in-binary-tree/
 * Discuss link: https://leetcode.com/problems/linked-list-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

use super::util::linked_list::{to_list, ListNode};
use super::util::tree::{to_tree, TreeNode};

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        /// ref https://leetcode.com/problems/linked-list-in-binary-tree/discuss/524881/Python-Recursive-Solution-O(N)-Time
        /// ref https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm
        let mut dp = vec![0];
        let mut i: usize = 0;
        let mut node = head.unwrap();
        let mut a = vec![node.val];

        while let Some(mut inner) = node.next {
            let node_val = inner.val;
            unsafe {
                while i != 0 && *a.get_unchecked(i) != node_val {
                    i = *dp.get_unchecked(i - 1);
                }
                i += (node_val == *a.get_unchecked(i)) as usize;
            }
            a.push(node_val);
            dp.push(i);
            node = inner;
        }

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut i: usize, a: &[i32], dp: &[usize]) -> bool {
            if let Some(inner) = root {
                let val = inner.as_ref().borrow().val;
                unsafe {
                    while i > 0 && val != *a.get_unchecked(i) {
                        i = *dp.get_unchecked(i - 1);
                    }
                    i += (val == *a.get_unchecked(i)) as usize;
                }
                if i == dp.len() {
                    return true;
                }
                dfs(inner.as_ref().borrow().left.clone(), i, a, dp)
                    || dfs(inner.as_ref().borrow().right.clone(), i, a, dp)
            } else {
                false
            }
        }

        dfs(root, 0, &a, &dp)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1367() {
        assert_eq!(
            Solution::is_sub_path(
                to_list(vec![4, 2, 8]),
                tree![1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]
            ),
            true
        );
        assert_eq!(
            Solution::is_sub_path(
                to_list(vec![1, 4, 2, 6]),
                tree![1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]
            ),
            true
        );
        assert_eq!(
            Solution::is_sub_path(
                to_list(vec![1, 4, 2, 6, 8]),
                tree![1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]
            ),
            false
        );
        assert_eq!(
            Solution::is_sub_path(
                to_list(vec![9, 1, 5, 7, 6, 5, 2, 1, 1, 2]),
                tree![
                    7, 4, 6, 7, 1, 9, 2, 1, 10, 7, null, 1, 9, 4, 7, null, null, null, 1, null, 2,
                    5, 5, 10, 10, null, null, 6, 1, null, 1, null, 6, 1, 6, 7, 4, 8, 7, null, null,
                    null, null, 10, 5, null, null, 2, 8, 7, 8, 4, null, 10, 6, 1, 7, 1, 9, null, 1,
                    null, null, null, null, null, null, null, null, 2, null, null, 7, null, 2, 9,
                    6, 5, 4, 6, 5, 9, 1, null, null, 7, null, null, null, null, null, 5, 3, 3,
                    null, 10, null, 3, 1, 2, null, 1, 2, 6, 6, 2, 1, 9, null, 8, 8, 5, null, null,
                    1, null, null, null, 3, null, null, null, 5, null, null, 1, 5, null, 1, 10,
                    null, 9, null, 5, 6, null, null, 10, null, 7, null, null, null, 4, null, null,
                    1, 4, 3, null, null, null, null, 8, 1, 8, 10, null, 1, null, null, null, null,
                    null, null, null, null, null, null, null, null, 7, 6, null, 7, null, null,
                    null, 5, null, null, null, 2, null, null, null, null, null, null, null, null,
                    null, null, null, null, null, null, 9
                ],
            ),
            true
        );
        assert_eq!(
            Solution::is_sub_path(
                to_list(vec![
                    1, 10, 3, 7, 10, 8, 9, 5, 3, 9, 6, 8, 7, 6, 6, 3, 5, 4, 4, 9, 6, 7, 9, 6, 9, 4,
                    9, 9, 7, 1, 5, 5, 10, 4, 4, 10, 7, 7, 2, 4, 5, 5, 2, 7, 5, 8, 6, 10, 2, 10, 1,
                    1, 6, 1, 8, 4, 7, 10, 9, 7, 9, 9, 7, 7, 7, 1, 5, 9, 8, 10, 5, 1, 7, 6, 1, 2,
                    10, 5, 7, 7, 2, 4, 10, 1, 7, 10, 9, 1, 9, 10, 4, 4, 1, 2, 1, 1, 3, 2, 6, 9
                ]),
                tree![
                    4, null, 8, null, 5, null, 7, null, 5, null, 2, 1, 3, null, null, null, 6, 8,
                    9, null, null, null, 3, null, 2, null, 10, null, 7, null, 8, 3, 4, null, null,
                    null, 3, 5, 1, null, null, null, 3, 1, 7, null, null, null, 4, 7, 7, null,
                    null, 8, 3, null, null, null, 6, 3, 1, null, null, null, 1, null, 8, null, 2,
                    5, 5, null, null, 1, 3, null, null, null, 5, null, 3, 3, 5, null, null, null,
                    7, null, 10, null, 7, null, 6, null, 8, null, 4, null, 10, null, 6, null, 6, 9,
                    3, null, null, 6, 5, null, null, null, 5, null, 2, null, 7, null, 5, null, 4,
                    8, 2, null, null, null, 2, null, 10, 10, 8, null, null, null, 7, null, 2, null,
                    5, 8, 6, null, null, null, 5, null, 7, null, 3, 4, 5, null, null, null, 4,
                    null, 8, null, 8, null, 8, null, 2, null, 5, 2, 9, null, null, null, 2, null,
                    3, 7, 1, null, null, 10, 1, null, null, null, 7, null, 6, null, 6, null, 7,
                    null, 7, null, 4, 4, 2, null, null, 7, 4, null, null, null, 7, null, 3, 7, 5,
                    null, null, null, 5, null, 4, null, 9, 5, 2, null, null, null, 4, null, 9,
                    null, 5, null, 5, null, 5, null, 2, null, 5, null, 2, null, 5, null, 7, 5, 5,
                    null, null, null, 6, null, 1, null, 7, null, 3, 9, 8, null, null, null, 4,
                    null, 7, 4, 8, null, null, 4, 2, null, null, null, 3, 10, 2, null, null, null,
                    7, null, 10, null, 3, null, 1, null, 2, null, 5, null, 9, null, 8, null, 5,
                    null, 9, null, 3, null, 7, null, 10, 5, 2, null, null, null, 2, 8, 10, null,
                    null, null, 4, 4, 7, null, null, null, 5, 1, 4, null, null, null, 10, null, 9,
                    null, 4, null, 9, 6, 5, null, null, null, 7, 5, 4, null, null, null, 8, null,
                    8, 4, 9, null, null, null, 6, 9, 1, null, null, null, 3, 3, 6, null, null,
                    null, 6, null, 7, null, 2, null, 1, null, 8, 2, 9, null, null, null, 8, null,
                    3, null, 1, 9, 1, null, null, null, 2, null, 6, null, 1, null, 6, 3, 9, null,
                    null, null, 10, null, 1, null, 9, null, 9, null, 10, null, 2, null, 6, null, 3,
                    null, 7, null, 2, null, 2, null, 2, 9, 5, null, null, null, 5, null, 6, null,
                    6, null, 2, null, 5, 7, 9, null, null, null, 6, 10, 4, null, null, 8, 4, null,
                    null, 4, 2, null, null, 4, 7, null, null, 2, 5, null, null, null, 4, 5, 1,
                    null, null, null, 3, null, 1, 10, 6, null, null, 3, 2, null, null, null, 6,
                    null, 9, null, 7, null, 5, 8, 5, null, null, null, 5, null, 5, 10, 6, null,
                    null, null, 7, null, 1, null, 6, 3, 7, null, null, null, 9, 7, 1, null, null,
                    null, 7, null, 4, null, 4, null, 9, null, 4, null, 1, null, 10, null, 1, 10,
                    10, null, null, null, 6, null, 3, null, 1, null, 9, null, 7, null, 6, 6, 1,
                    null, null, null, 9, 4, 7, null, null, null, 3, null, 10, null, 4, 3, 3, null,
                    null, null, 4, 5, 10, null, null, null, 1, 8, 10, null, null, null, 6, null, 9,
                    null, 10, null, 4, 4, 9, null, null, null, 3, null, 3, null, 3, null, 10, null,
                    10, null, 6, 8, 1, null, null, null, 9, 7, 1, null, null, null, 5, null, 3,
                    null, 10, null, 5, null, 9, null, 5, null, 8, null, 6, 3, 2, null, null, null,
                    8, null, 8, 3, 9, null, null, null, 9, null, 10, 3, 8, null, null, 6, 6, null,
                    null, null, 6, null, 8, null, 2, null, 9, null, 4, null, 6, null, 4, null, 4,
                    null, 6, null, 9, null, 7, null, 10, null, 1, null, 3, null, 6, null, 7, null,
                    4, null, 9, null, 1, null, 3, 8, 10, null, null, null, 2, null, 10, null, 4,
                    null, 8, null, 10, null, 7, null, 8, 5, 1, null, null, 9, 3, null, null, 7, 8,
                    null, null, null, 1, null, 1, 5, 4, null, null, null, 1, null, 4, 5, 7, null,
                    null, null, 3, null, 6, null, 6, null, 9, null, 4, null, 1, 5, 10, null, null,
                    null, 3, null, 7, null, 10, null, 8, null, 9, 2, 5, null, null, null, 3, null,
                    9, 10, 6, null, null, null, 8, null, 7, 8, 6, null, null, null, 6, null, 3,
                    null, 5, null, 4, null, 4, null, 9, null, 6, 2, 7, null, null, null, 9, null,
                    6, 1, 9, null, null, null, 4, null, 9, 9, 9, null, null, null, 7, 7, 1, null,
                    null, null, 5, null, 5, 6, 10, null, null, null, 4, null, 4, 10, 10, null,
                    null, null, 7, 2, 7, null, null, null, 2, null, 4, null, 5, null, 5, 10, 2,
                    null, null, null, 7, 9, 5, null, null, null, 8, null, 6, null, 10, 8, 2, null,
                    null, 8, 10, null, null, null, 1, null, 1, null, 6, 5, 1, null, null, 8, 8,
                    null, null, 8, 4, null, null, null, 7, null, 10, 4, 9, null, null, null, 7,
                    null, 9, null, 9, 1, 7, null, null, 4, 7, null, null, null, 7, null, 1, null,
                    5, 8, 9, null, null, 9, 8, null, null, 9, 10, null, null, 4, 5, null, null, 1,
                    1, null, null, null, 7, null, 6, null, 1, null, 2, 1, 10, null, null, 2, 5,
                    null, null, 7, 7, null, null, null, 7, null, 2, null, 4, 3, 10, null, null,
                    null, 1, null, 7, null, 10, 7, 9, null, null, null, 5, 4, 9, null, null, null,
                    10, 6, 4, null, null, 8, 4, null, null, null, 1, null, 2, null, 1, 8, 1, null,
                    null, null, 3, null, 2, null, 6, null, 9, null, 2, 1, 10, null, null, null, 5,
                    null, 8, 2, 1, null, null, null, 2, 3, 10, null, null, null, 8, null, 9, null,
                    5, null, 4, null, 1, 9, 10, null, null, 4, 9, null, null, 3, 5, null, null,
                    null, 6, null, 6, 9, 1, null, null, null, 5, null, 2, null, 2, null, 6, null,
                    1, 7, 9, null, null, null, 6, null, 8, 4, 4, null, null, null, 2, null, 10,
                    null, 1, null, 2, null, 9, null, 8, null, 2, null, 1, 10, 4, null, null, null,
                    10, null, 8, 3, 2, null, null, null, 10, null, 3, 8, 1, null, null, 5, 3, null,
                    null, null, 6, null, 8, null, 7, 2, 5, null, null, 1, 6, null, null, null, 8,
                    null, 6, null, 3, null, 8, null, 9, null, 5, null, 2, null, 9, null, 2, 6, 10,
                    null, null, 7, 10, null, null, null, 6, null, 8, null, 7, 7, 4, null, null,
                    null, 3, 5, 2, null, null, 10, 4, null, null, null, 4, 4, 3, null, null, null,
                    5, null, 1, null, 10, null, 10, null, 5, null, 9, null, 3, null, 8, null, 3,
                    null, 2, null, 4, 1, 1, null, null, null, 7, 10, 8, null, null, null, 9, 4, 8,
                    null, null, 1, 2, null, null, 9, 7, null, null, 5, 8, null, null, null, 9,
                    null, 7, null, 4, null, 4, 5, 3, null, null, null, 2, null, 4, 3, 10, null,
                    null, 7, 7, null, null, null, 2, null, 2, 8, 8, null, null, null, 2, null, 4,
                    null, 5, 8, 4, null, null, null, 9, null, 4, null, 10, null, 4, null, 5, null,
                    5, null, 1, null, 5, null, 8, null, 5, null, 5, null, 1, null, 10, null, 9,
                    null, 10, null, 2, null, 7, 5, 9, null, null, null, 6, 4, 6, null, null, null,
                    2, null, 10, null, 1, 4, 3, null, null, 7, 8, null, null, null, 3, null, 3,
                    null, 8, null, 10, null, 6, 6, 10, null, null, null, 1, 8, 5, null, null, 1, 3,
                    null, null, null, 8, null, 9, null, 10, null, 8, 4, 9, null, null, 10, 1, null,
                    null, null, 2, null, 8, 5, 2, null, null, 8, 6, null, null, null, 4, null, 7,
                    10, 1, null, null, null, 3, 3, 3, null, null, null, 3, null, 5, 7, 3, null,
                    null, 10, 9, null, null, null, 2, null, 8, null, 10, null, 7, null, 10, null,
                    3, 9, 10, null, null, null, 6, 4, 9, null, null, 9, 3, null, null, null, 7,
                    null, 2, null, 10, null, 10, null, 7, null, 4, 5, 7, null, null, 9, 8, null,
                    null, null, 6, 3, 1, null, null, null, 9, null, 7, 4, 4, null, null, null, 6,
                    null, 1, null, 9, null, 9, null, 3, 1, 1, null, null, 1, 8, null, null, null,
                    1, null, 2, null, 7, 4, 6, null, null, null, 1, null, 3, null, 8, null, 10,
                    null, 3, null, 10, null, 10, null, 10, null, 10, null, 10, null, 6, null, 7,
                    null, 3, null, 9, null, 7, 5, 4, null, null, null, 5, null, 5, 1, 3, null,
                    null, null, 6, 3, 4, null, null, null, 3, 2, 10, null, null, 10, 5, null, null,
                    null, 5, 9, 1, null, null, null, 8, null, 7, null, 9, 5, 3, null, null, null,
                    2, null, 7, null, 10, null, 2, 9, 4, null, null, null, 4, 10, 10, null, null,
                    null, 6, 2, 6, null, null, null, 4, null, 5, null, 7, null, 7, null, 2, 4, 1,
                    null, null, null, 7, null, 5, 8, 8, null, null, 3, 6, null, null, null, 1,
                    null, 5, null, 8, 4, 6, null, null, null, 6, null, 9, null, 4, null, 4, null,
                    3, null, 2, 6, 9, null, null, null, 6, 6, 8, null, null, null, 7, null, 5,
                    null, 5, null, 9, 4, 3, null, null, null, 10, 4, 6, null, null, null, 9, null,
                    3, null, 10, null, 9, null, 1, null, 6, null, 1, null, 4, null, 5, 5, 3, null,
                    null, 7, 8, null, null, null, 6, null, 6, null, 5, 9, 4, null, null, null, 9,
                    null, 7, null, 7, 7, 5, null, null, null, 7, null, 3, 8, 3, null, null, null,
                    1, 5, 4, null, null, null, 2, null, 3, null, 4, null, 5, 5, 6, null, null,
                    null, 2, null, 2, 7, 9, null, null, 9, 5, null, null, null, 9, null, 9, null,
                    8, 7, 6, null, null, null, 2, null, 9, null, 2, null, 7, 6, 4, null, null,
                    null, 1, null, 7, null, 2, null, 7, null, 3, 9, 2, null, null, 4, 5, null,
                    null, null, 3, null, 2, null, 8, 7, 8, null, null, 7, 7, null, null, null, 10,
                    null, 9, 2, 7, null, null, 6, 3, null, null, null, 10, null, 5, null, 7, null,
                    9, null, 3, null, 1, null, 9, null, 2, 5, 2, null, null, null, 4, null, 8,
                    null, 6, 10, 10, null, null, 10, 3, null, null, null, 3, null, 1, null, 3,
                    null, 8, 3, 2, null, null, null, 5, 2, 8, null, null, 7, 5, null, null, 7, 7,
                    null, null, null, 1, 6, 5, null, null, null, 2, null, 4, null, 7, null, 5,
                    null, 3, null, 7, null, 10, null, 10, null, 7, null, 9, null, 5, 5, 5, null,
                    null, null, 9, null, 4, null, 7, null, 6, null, 2, null, 3, null, 3, 8, 10,
                    null, null, null, 1, null, 3, null, 6, null, 10, null, 8, null, 6, 4, 5, null,
                    null, null, 6, null, 3, null, 3, null, 8, 1, 3, null, null, 2, 3, null, null,
                    null, 7, null, 10, null, 2, null, 10, null, 2, null, 7, null, 10, 6, 7, null,
                    null, 3, 4, null, null, 6, 2, null, null, null, 9, null, 8, null, 7, null, 10,
                    null, 9, 10, 1, null, null, null, 5, 1, 10, null, null, 10, 2, null, null,
                    null, 2, 5, 8, null, null, null, 9, 8, 8, null, null, null, 8, null, 4, 1, 3,
                    null, null, null, 4, null, 4, null, 9, null, 4, 10, 7, null, null, 10, 4, null,
                    null, 4, 5, null, null, 9, 2, null, null, 3, 7, null, null, 8, 7, null, null,
                    null, 5, null, 10, null, 3, null, 8, null, 3, null, 5, 2, 9, null, null, null,
                    10, null, 3, null, 10, null, 7, 5, 1, null, null, 2, 4, null, null, null, 5,
                    null, 2, null, 6, null, 8, null, 9, null, 10, null, 9, null, 6, null, 2, 6, 7,
                    null, null, 2, 7, null, null, null, 3, null, 6, 9, 5, null, null, 2, 6, null,
                    null, null, 8, null, 4, null, 8, null, 2, 4, 9, null, null, 4, 7, null, null,
                    null, 9, null, 5, null, 3, null, 8, null, 6, null, 5, 7, 4, null, null, 8, 7,
                    null, null, null, 9, 1, 2, null, null, null, 9, 6, 7, null, null, null, 8,
                    null, 6, null, 6, 4, 6, null, null, null, 3, null, 5, 10, 4, null, null, null,
                    5, null, 8, null, 8, null, 7, null, 10, 5, 10, null, null, null, 10, null, 10,
                    null, 10, 8, 2, null, null, 5, 3, null, null, null, 8, 6, 6, null, null, 10, 8,
                    null, null, 1, 8, null, null, null, 9, 1, 6, null, null, 7, 6, null, null,
                    null, 10, 5, 4, null, null, null, 10, 5, 4, null, null, 2, 5, null, null, null,
                    4, 2, 5, null, null, null, 3, null, 4, 2, 8, null, null, null, 5, null, 9,
                    null, 3, 9, 3, null, null, null, 5, null, 7, null, 7, null, 5, null, 10, null,
                    3, 2, 7, null, null, 3, 8, null, null, null, 10, 2, 3, null, null, null, 7, 3,
                    3, null, null, null, 6, null, 4, null, 8, null, 3, null, 3, null, 1, null, 9,
                    10, 1, null, null, null, 1, null, 6, 6, 5, null, null, 6, 3, null, null, null,
                    6, null, 4, null, 2, null, 10, null, 9, 2, 5, null, null, null, 10, null, 10,
                    3, 5, null, null, 10, 6, null, null, 1, 9, null, null, 6, 7, null, null, 6, 5,
                    null, null, null, 8, null, 8, 5, 6, null, null, null, 6, null, 8, null, 8,
                    null, 4, null, 6, null, 9, null, 2, null, 1, null, 10, null, 9, null, 9, null,
                    4, 1, 6, null, null, null, 1, null, 3, null, 4, 10, 8, null, null, null, 7,
                    null, 5, null, 10, null, 1, null, 9, null, 9, null, 9, 1, 8, null, null, null,
                    1, null, 9, 5, 1, null, null, 7, 1, null, null, null, 8, null, 1, 8, 6, null,
                    null, 2, 9, null, null, 10, 5, null, null, null, 2, null, 10, null, 10, null,
                    9, null, 10, null, 7, null, 7, null, 5, null, 8, 8, 2, null, null, null, 9,
                    null, 10, null, 1, null, 1, null, 1, null, 10, 6, 1, null, null, null, 9, null,
                    2, 9, 9, null, null, null, 9, 3, 8, null, null, null, 1, null, 10, 1, 10, null,
                    null, null, 8, null, 7, null, 8, null, 8, 6, 5, null, null, 2, 5, null, null,
                    null, 7, null, 1, null, 10, null, 4, 8, 5, null, null, 5, 2, null, null, 2, 3,
                    null, null, null, 6, 3, 10, null, null, 1, 8, null, null, null, 9, null, 8, 7,
                    10, null, null, null, 10, null, 5, 10, 6, null, null, null, 5, null, 6, null,
                    5, 6, 6, null, null, 5, 8, null, null, null, 7, null, 8, null, 10, 1, 1, null,
                    null, null, 10, 1, 2, null, null, 9, 5, null, null, null, 7, 4, 5, null, null,
                    null, 10, null, 3, null, 5, null, 8, null, 2, null, 9, null, 9, 6, 7, null,
                    null, 7, 1, null, null, null, 5, null, 2, null, 8, 5, 3, null, null, null, 7,
                    null, 6, null, 6, null, 7, null, 5, null, 1, 6, 7, null, null, null, 6, null,
                    8, null, 8, null, 5, 10, 10, null, null, null, 10, 5, 2, null, null, 6, 5,
                    null, null, 8, 1, null, null, 2, 3, null, null, 9, 3, null, null, 10, 7, null,
                    null, 1, 4, null, null, 5, 10, null, null, null, 7, null, 6, null, 1, null, 9,
                    null, 8, null, 2, 10, 7, null, null, null, 5, 3, 9, null, null, null, 2, null,
                    7, null, 3, null, 7, null, 7, 9, 2, null, null, null, 5, null, 6, 1, 2, null,
                    null, 5, 10, null, null, 6, 9, null, null, null, 10, 9, 8, null, null, 5, 9,
                    null, null, null, 10, 5, 6, null, null, null, 10, 10, 1, null, null, null, 7,
                    null, 10, null, 3, null, 2, null, 6, 9, 9, null, null, 2, 5, null, null, null,
                    1, null, 8, null, 2, null, 4, 2, 9, null, null, null, 10, null, 6, null, 5, 2,
                    3, null, null, null, 1, null, 7, null, 10, 6, 10, null, null, null, 2, 5, 9,
                    null, null, 4, 7, null, null, null, 2, 1, 1, null, null, null, 9, null, 5, 7,
                    7, null, null, null, 3, null, 4, null, 10, 2, 6, null, null, 8, 6, null, null,
                    1, 10, null, null, null, 10, 4, 4, null, null, null, 7, null, 8, 7, 5, null,
                    null, null, 2, 10, 6, null, null, 3, 6, null, null, null, 10, null, 8, null, 8,
                    8, 9, null, null, null, 7, null, 8, null, 1, null, 5, null, 8, null, 7, 10, 6,
                    null, null, null, 3, null, 5, null, 6, 9, 10, null, null, null, 10, null, 6,
                    null, 2, null, 2, null, 2, null, 9, null, 7, null, 4, 5, 9, null, null, null,
                    4, null, 4, null, 3, null, 10, null, 3, 10, 3, null, null, 5, 7, null, null,
                    null, 6, null, 3, 3, 4, null, null, null, 7, null, 6, null, 10, null, 5, 8, 8,
                    null, null, null, 4, 5, 5, null, null, null, 2, null, 10, null, 2, null, 1, 2,
                    8, null, null, null, 5, null, 8, null, 3, null, 4, null, 8, null, 1, null, 5,
                    8, 1, null, null, 3, 9, null, null, null, 3, 1, 1, null, null, 5, 9, null,
                    null, null, 6, null, 9, 5, 6, null, null, null, 5, 3, 5, null, null, null, 9,
                    3, 1, null, null, 3, 5, null, null, 3, 10, null, null, null, 10, null, 8, null,
                    1, null, 7, null, 4, null, 1, null, 7, null, 1, null, 3, 7, 9, null, null, 1,
                    2, null, null, null, 8, 3, 7, null, null, null, 8, null, 1, 6, 6, null, null,
                    null, 9, 7, 4, null, null, 6, 10, null, null, 4, 5, null, null, null, 1, null,
                    7, null, 6, 7, 3, null, null, null, 6, null, 9, null, 8, 2, 6, null, null, 6,
                    8, null, null, 2, 7, null, null, null, 8, null, 8, 7, 5, null, null, null, 4,
                    null, 9, 5, 3, null, null, 9, 5, null, null, null, 5, null, 1, null, 5, null,
                    6, 8, 6, null, null, null, 5, null, 4, null, 2, 8, 5, null, null, null, 9,
                    null, 5, null, 9, null, 3, null, 5, 9, 3, null, null, null, 2, null, 7, null,
                    8, null, 8, null, 8, null, 10, 7, 2, null, null, null, 6, null, 2, 1, 10, null,
                    null, null, 6, null, 8, null, 4, null, 6, 8, 5, null, null, null, 3, null, 1,
                    null, 6, null, 6, null, 2, null, 9, 1, 9, null, null, null, 3, null, 7, 4, 7,
                    null, null, 9, 6, null, null, 7, 8, null, null, null, 1, 5, 1, null, null, 7,
                    10, null, null, null, 6, null, 8, 3, 2, null, null, 1, 5, null, null, null, 8,
                    null, 3, null, 3, 9, 1, null, null, null, 8, null, 1, 3, 5, null, null, null,
                    9, null, 8, 3, 4, null, null, null, 9, null, 1, null, 3, null, 7, null, 3, 5,
                    1, null, null, null, 4, null, 1, null, 5, null, 1, null, 3, 4, 8, null, null,
                    null, 1, 10, 7, null, null, null, 1, null, 9, null, 7, null, 3, null, 10, 6, 9,
                    null, null, null, 3, 6, 8, null, null, null, 8, null, 3, null, 4, null, 10,
                    null, 2, 10, 7, null, null, 5, 4, null, null, null, 4, 2, 6, null, null, 1, 10,
                    null, null, null, 4, 3, 7, null, null, null, 4, null, 1, null, 6, null, 10,
                    null, 7, 4, 9, null, null, null, 10, 9, 4, null, null, null, 6, 5, 9, null,
                    null, null, 7, 1, 7, null, null, null, 4, null, 4, null, 4, null, 6, 4, 3,
                    null, null, null, 4, null, 5, null, 10, null, 2, null, 1, null, 1, null, 2,
                    null, 2, 9, 4, null, null, null, 9, null, 9, 9, 4, null, null, null, 5, null,
                    6, null, 2, null, 3, null, 10, 9, 10, null, null, 10, 2, null, null, 3, 9,
                    null, null, null, 9, null, 10, null, 9, null, 3, null, 1, 5, 6, null, null,
                    null, 6, null, 2, null, 9, null, 3, null, 9, 9, 3, null, null, 5, 3, null,
                    null, null, 2, null, 3, null, 8, null, 2, null, 9, null, 3, null, 4, null, 3,
                    null, 4, null, 8, 6, 7, null, null, null, 6, null, 3, null, 1, null, 9, 5, 1,
                    null, null, null, 2, null, 7, 4, 7, null, null, null, 2, null, 9, null, 7,
                    null, 10, null, 6, null, 7, null, 1, null, 4, null, 5, null, 2, null, 7, null,
                    3, null, 7, null, 4, null, 5, null, 10, null, 1, null, 9, 9, 8, null, null, 10,
                    10, null, null, null, 6, 6, 10, null, null, 10, 4, null, null, 4, 6, null,
                    null, null, 4, null, 3, null, 5, 4, 8, null, null, null, 5, 6, 3, null, null,
                    1, 7, null, null, 9, 4, null, null, null, 9, 10, 2, null, null, null, 5, null,
                    6, 2, 5, null, null, null, 10, 5, 1, null, null, null, 8, 2, 2, null, null, 7,
                    6, null, null, null, 9, null, 4, null, 4, null, 2, null, 4, null, 8, 1, 10,
                    null, null, null, 8, null, 3, null, 1, null, 5, null, 2, null, 9, 8, 5, null,
                    null, 8, 6, null, null, null, 1, null, 6, null, 2, 2, 9, null, null, null, 9,
                    5, 7, null, null, null, 4, null, 5, 4, 5, null, null, 1, 1, null, null, 8, 3,
                    null, null, null, 10, 7, 10, null, null, 6, 5, null, null, 6, 3, null, null, 4,
                    1, null, null, 10, 1, null, null, 4, 2, null, null, 6, 3, null, null, null, 2,
                    null, 9, null, 10, 9, 9, null, null, null, 2, null, 8, null, 8, 6, 2, null,
                    null, 10, 7, null, null, null, 10, 1, 3, null, null, 2, 3, null, null, null,
                    10, 3, 1, null, null, null, 9, null, 4, null, 3, null, 4, null, 7, null, 2,
                    null, 1, null, 9, null, 1, null, 7, null, 9, null, 7, null, 6, 7, 9, null,
                    null, null, 10, null, 6, 3, 2, null, null, null, 4, null, 4, null, 5, 4, 1,
                    null, null, null, 3, null, 3, null, 6, null, 5, null, 4, 10, 5, null, null, 4,
                    6, null, null, 10, 4, null, null, null, 7, null, 10, null, 1, null, 1, 5, 6,
                    null, null, 9, 7, null, null, null, 3, null, 6, null, 8, null, 2, null, 4,
                    null, 2, null, 7, null, 8, 3, 10, null, null, null, 6, null, 3, null, 7, null,
                    4, 2, 3, null, null, 1, 9, null, null, 5, 6, null, null, 6, 6, null, null,
                    null, 7, null, 8, 9, 9, null, null, null, 9, null, 1, null, 9, null, 5, null,
                    1, null, 5, 2, 6, null, null, null, 9, 2, 4, null, null, 3, 6, null, null, 4,
                    2, null, null, null, 9, null, 6, 3, 3, null, null, null, 7, null, 9, null, 6,
                    2, 9, null, null, null, 8, null, 5, null, 4, null, 7, null, 4, null, 8, null,
                    5, 3, 2, null, null, null, 1, null, 1, null, 1, null, 1, null, 1, null, 1,
                    null, 4, null, 6, 3, 7, null, null, 9, 7, null, null, 9, 2, null, null, null,
                    4, null, 1, null, 5, null, 8, null, 2, 10, 1, null, null, null, 10, null, 1, 2,
                    7, null, null, null, 5, null, 8, null, 7, 2, 6, null, null, null, 10, null, 3,
                    null, 7, null, 3, null, 6, 9, 4, null, null, null, 2, null, 8, null, 8, null,
                    1, null, 8, null, 8, null, 9, null, 7, null, 5, 10, 9, null, null, 4, 4, null,
                    null, null, 7, null, 6, null, 8, 1, 3, null, null, 9, 3, null, null, 1, 10,
                    null, null, null, 9, 8, 2, null, null, null, 8, null, 4, null, 4, 4, 1, null,
                    null, null, 7, null, 8, 1, 9, null, null, null, 10, null, 3, 6, 7, null, null,
                    null, 5, 5, 2, null, null, null, 4, null, 5, 6, 6, null, null, 7, 7, null,
                    null, 5, 10, null, null, null, 6, 10, 6, null, null, null, 2, null, 5, 2, 5,
                    null, null, null, 7, null, 7, null, 7, null, 4, null, 9, null, 4, null, 8,
                    null, 1, null, 5, null, 9, 6, 4, null, null, null, 8, 9, 2, null, null, 10, 2,
                    null, null, null, 3, null, 4, null, 10, null, 6, null, 10, 2, 9, null, null, 6,
                    1, null, null, null, 7, 6, 6, null, null, null, 2, null, 4, null, 10, 8, 9,
                    null, null, null, 7, 3, 9, null, null, 10, 4, null, null, null, 10, null, 6,
                    null, 2, null, 5, null, 1, null, 8, 8, 3, null, null, null, 2, 5, 10, null,
                    null, 5, 8, null, null, 3, 10, null, null, null, 5, null, 8, null, 5, null, 4,
                    null, 5, 6, 2, null, null, null, 7, null, 5, null, 10, null, 8, 1, 5, null,
                    null, null, 1, null, 1, null, 5, null, 9, null, 6, null, 1, null, 5
                ],
            ),
            false
        );
    }
}
