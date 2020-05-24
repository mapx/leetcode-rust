/**
 * [700] Search in a Binary Search Tree
 *
 * Given the root node of a binary search tree (BST) and a value. You need to find the node in the BST that the node's value equals the given value. Return the subtree rooted with that node. If such node doesn't exist, you should return NULL.
 *
 * For example,
 *
 *
 * Given the tree:
 *         4
 *        / \
 *       2   7
 *      / \
 *     1   3
 *
 * And the value to search: 2
 *
 *
 * You should return this subtree:
 *
 *
 *       2     
 *      / \   
 *     1   3
 *
 *
 * In the example above, if we want to search the value `5`, since there is no node with value `5`, we should return `NULL`.
 *
 * Note that an empty tree is represented by `NULL`, therefore you would see the expected output (serialized tree format) as `[]`, not `null`.
 *
 * Problem link: https://leetcode.com/problems/search-in-a-binary-search-tree/
 * Discuss link: https://leetcode.com/problems/search-in-a-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}
use super::util::tree::{to_tree, TreeNode};

// submission codes start here

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
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let node = node.borrow();
            return match node.val.cmp(&val) {
                Ordering::Less => Self::search_bst(node.right.clone(), val),
                Ordering::Greater => Self::search_bst(node.left.clone(), val),
                Ordering::Equal => root.clone(),
            };
        }
        None
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_700() {
        assert_eq!(
            Solution::search_bst(tree![4, 2, 7, 1, 3], 2),
            tree![2, 1, 3]
        );
        assert_eq!(Solution::search_bst(tree![4, 2, 7, 1, 5], 5), tree![]);
    }
}
