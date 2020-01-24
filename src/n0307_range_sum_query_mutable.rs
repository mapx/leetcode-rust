/**
 * [307] Range Sum Query - Mutable
 *
 * Given an integer array nums, find the sum of the elements between indices i and j (i &le; j), inclusive.
 *
 * The update(i, val) function modifies nums by updating the element at index i to val.
 *
 * Example:
 *
 *
 * Given nums = [1, 3, 5]
 *
 * sumRange(0, 2) -> 9
 * update(1, 2)
 * sumRange(0, 2) -> 8
 *
 *
 * Note:
 *
 * <ol>
 * 	The array is only modifiable by the update function.
 * 	You may assume the number of calls to update and sumRange function is distributed evenly.
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here

use std::usize;

struct NumArray {
    tree: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut arr = NumArray { tree: vec![0; n] };
        arr.tree.extend_from_slice(&nums);
        for i in (1..n).rev() {
            unsafe {
                *arr.tree.get_unchecked_mut(i) =
                    *arr.tree.get_unchecked(i * 2) + *arr.tree.get_unchecked(i * 2 + 1);
            }
        }
        arr
    }

    fn update(&mut self, i: i32, val: i32) {
        let mut i = (i as usize) + self.tree.len() / 2;
        unsafe {
            *self.tree.get_unchecked_mut(i) = val;
            while i > 0 {
                let left = i & (usize::MAX - 1);
                i >>= 1;
                *self.tree.get_unchecked_mut(i) =
                    *self.tree.get_unchecked(left) + *self.tree.get_unchecked(left + 1);
            }
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let n = self.tree.len() / 2;
        let mut total = 0;
        let mut i = (i as usize) + n;
        let mut j = (j as usize) + n;
        while i <= j {
            if i & 1 != 0 {
                unsafe {
                    total += *self.tree.get_unchecked(i);
                }
                i += 1;
            }
            if j & 1 == 0 {
                unsafe {
                    total += *self.tree.get_unchecked(j);
                }
                j -= 1;
            }
            i >>= 1;
            j >>= 1;
        }
        total as i32
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(i, val);
 * let ret_2: i32 = obj.sum_range(i, j);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_307() {
        let mut obj = NumArray::new(vec![1, 3, 5]);
        assert_eq!(obj.sum_range(0, 2), 9);
        obj.update(1, 2);
        assert_eq!(obj.sum_range(0, 2), 8);
    }
}
