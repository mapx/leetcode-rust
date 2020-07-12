/**
 * [1491] Average Salary Excluding the Minimum and Maximum Salary
 *
 * Given an array of unique integers `salary` where `salary[i]` is the salary of the employee `i`.
 *
 * Return the average salary of employees excluding the minimum and maximum salary.
 *
 *  
 * Example 1:
 *
 *
 * Input: salary = [4000,3000,1000,2000]
 * Output: 2500.00000
 * Explanation: Minimum salary and maximum salary are 1000 and 4000 respectively.
 * Average salary excluding minimum and maximum salary is (2000+3000)/2= 2500
 *
 *
 * Example 2:
 *
 *
 * Input: salary = [1000,2000,3000]
 * Output: 2000.00000
 * Explanation: Minimum salary and maximum salary are 1000 and 3000 respectively.
 * Average salary excluding minimum and maximum salary is (2000)/1= 2000
 *
 *
 * Example 3:
 *
 *
 * Input: salary = [6000,5000,4000,3000,2000,1000]
 * Output: 3500.00000
 *
 *
 * Example 4:
 *
 *
 * Input: salary = [8000,9000,2000,3000,6000,1000]
 * Output: 4750.00000
 *
 *
 *  
 * Constraints:
 *
 *
 *     `3 <= salary.length <= 100`
 *     `10^3 <= salary[i] <= 10^6`
 *     `salary[i]` is unique.
 *     Answers within `10^-5` of the actual value will be accepted as correct.
 *
 * Problem link: https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/
 * Discuss link: https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/discuss/?currentPage=1&orderBy=most_votes
 */
pub struct Solution {}

// submission codes start here
use assert_approx_eq::assert_approx_eq;
use std::cmp;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut total = 0f64;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let it = salary.iter();
        salary.iter().for_each(|&n| {
            min = cmp::min(min, n);
            max = cmp::max(max, n);
            total += n as f64;
        });
        (total - (min as f64) - (max as f64)) / (salary.len() - 2) as f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1491() {
        assert_approx_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.0);
        assert_approx_eq!(Solution::average(vec![1000, 2000, 3000]), 2000.0);
        assert_approx_eq!(
            Solution::average(vec![6000, 5000, 4000, 3000, 2000, 1000]),
            3500.0
        );
        assert_approx_eq!(
            Solution::average(vec![8000, 9000, 2000, 3000, 6000, 1000]),
            4750.0
        );
    }
}
