/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 * Example:
 *
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 *
 *
 */
pub struct Solution {}
use super::util::linked_list::{to_list, ListNode};

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
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let (mut l1, mut l2) = (l1, l2);
        let mut carry = 0;
        let mut current = head.as_mut();
        while l1.is_some() || l2.is_some() {
            let x = match l1 {
                Some(v) => {
                    l1 = v.next;
                    v.val
                }
                None => 0,
            };
            let y = match l2 {
                Some(v) => {
                    l2 = v.next;
                    v.val
                }
                None => 0,
            };
            let sum = x + y + carry;
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            unsafe {
                current = (*(*(current as *mut ListNode)).next.as_mut().unwrap()).as_mut();
            }
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(1)));
        }

        head.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
