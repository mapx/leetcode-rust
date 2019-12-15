/**
 * [1200] Remove Interval
 *
 * Given a sorted list of disjoint intervals, each interval intervals[i] = [a, b] represents the set of real numbers x such that a <= x < b.
 * We remove the intersections between any interval in intervals and the interval toBeRemoved.
 * Return a sorted list of intervals after all such removals.
 *  
 * Example 1:
 * Input: intervals = [[0,2],[3,4],[5,7]], toBeRemoved = [1,6]
 * Output: [[0,1],[6,7]]
 * Example 2:
 * Input: intervals = [[0,5]], toBeRemoved = [2,3]
 * Output: [[0,2],[3,5]]
 *  
 * Constraints:
 *
 * 	1 <= intervals.length <= 10^4
 * 	-10^9 <= intervals[i][0] < intervals[i][1] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here
use std::cmp;

impl Solution {
    pub fn remove_interval(intervals: Vec<Vec<i32>>, to_be_removed: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        if let [lo, hi] = &to_be_removed[..] {
            for interval in intervals {
                let a = unsafe { interval.get_unchecked(0) };
                let b = unsafe { interval.get_unchecked(1) };
                if b <= lo || hi <= a {
                    ret.push(vec![*a, *b]);
                } else {
                    let i = cmp::min(lo, b);
                    if a < i {
                        ret.push(vec![*a, *i]);
                    }
                    let j = cmp::max(hi, a);
                    if j < b {
                        ret.push(vec![*j, *b]);
                    }
                }
            }
            return ret;
        }
        intervals
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1200() {
        assert_eq!(
            Solution::remove_interval(vec![vec![0, 2], vec![3, 4], vec![5, 7]], vec![1, 6]),
            vec![vec![0, 1], vec![6, 7]]
        );
        assert_eq!(
            Solution::remove_interval(vec![vec![0, 5]], vec![2, 3]),
            vec![vec![0, 2], vec![3, 5]]
        );
    }
}
