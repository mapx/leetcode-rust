/**
 * [1334] Find the City With the Smallest Number of Neighbors at a Threshold Distance
 *
 * There are n cities numbered from 0 to n-1. Given the array edges where edges[i] = [fromi, toi, weighti] represents a bidirectional and weighted edge between cities fromi and toi, and given the integer distanceThreshold.
 *
 * Return the city with the smallest number of cities that are reachable through some path and whose distance is at most distanceThreshold, If there are multiple such cities, return the city with the greatest number.
 *
 * Notice that the distance of a path connecting cities i and j is equal to the sum of the edges' weights along that path.
 *
 *  
 * Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/find_the_city_01.png" style="width: 300px; height: 225px;" />
 *
 *
 * Input: n = 4, edges = [[0,1,3],[1,2,1],[1,3,4],[2,3,1]], distanceThreshold = 4
 * Output: 3
 * Explanation: The figure above describes the graph.
 * The neighboring cities at a distanceThreshold = 4 for each city are:
 * City 0 -> [City 1, City 2]
 * City 1 -> [City 0, City 2, City 3]
 * City 2 -> [City 0, City 1, City 3]
 * City 3 -> [City 1, City 2]
 * Cities 0 and 3 have 2 neighboring cities at a distanceThreshold = 4, but we have to return city 3 since it has the greatest number.
 *
 *
 * Example 2:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/find_the_city_02.png" style="width: 300px; height: 225px;" />
 *
 *
 * Input: n = 5, edges = [[0,1,2],[0,4,8],[1,2,3],[1,4,2],[2,3,1],[3,4,1]], distanceThreshold = 2
 * Output: 0
 * Explanation: The figure above describes the graph.
 * The neighboring cities at a distanceThreshold = 2 for each city are:
 * City 0 -> [City 1]
 * City 1 -> [City 0, City 4]
 * City 2 -> [City 3, City 4]
 * City 3 -> [City 2, City 4]
 * City 4 -> [City 1, City 2, City 3]
 * The city 0 has 1 neighboring city at a distanceThreshold = 2.
 *
 *
 *  
 * Constraints:
 *
 *
 *     2 <= n <= 100
 *     1 <= edges.length <= n * (n - 1) / 2
 *     edges[i].length == 3
 *     0 <= fromi < toi < n
 *     1 <= weighti, distanceThreshold <= 10^4
 *     All pairs (fromi, toi) are distinct.
 *
 */
pub struct Solution {}

// submission codes start here

use std::cmp;
use std::collections::HashMap;
use std::mem;

impl Solution {
    // Time complexity: O(n^3), Space complexity: O(n^2)
    #[allow(clippy::many_single_char_names)]
    fn floyd_warshall_algorithm(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let distance_threshold = distance_threshold as u16;
        let mut g = vec![10001u16; n * n]; // init the graph, with default remain distance capability
        for edge in edges.iter() {
            unsafe {
                let a = *edge.get_unchecked(0) as usize;
                let b = *edge.get_unchecked(1) as usize;
                let w = *edge.get_unchecked(2) as u16;
                *g.get_unchecked_mut(a + b * n) = w;
                *g.get_unchecked_mut(a * n + b) = w;
            }
        }
        (0..n).for_each(|i| unsafe {
            *g.get_unchecked_mut(i * n + i) = 0;
        });

        // update the distances with Floyd-Warshall algorithm
        // ref https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    unsafe {
                        let d = cmp::min(
                            *g.get_unchecked(i * n + j),
                            *g.get_unchecked(i * n + k) + *g.get_unchecked(k * n + j),
                        );
                        *g.get_unchecked_mut(i * n + j) = d;
                        *g.get_unchecked_mut(j * n + i) = d;
                    }
                }
            }
        }

        let mut best_count = n;
        let mut best_city = 0;
        (0..n).for_each(|i| {
            let count = g[i * n..(i + 1) * n]
                .iter()
                .map(|d| (*d <= distance_threshold) as usize)
                .sum();
            if count <= best_count {
                best_count = count;
                best_city = i;
            }
        });
        best_city as i32
    }

    #[allow(clippy::many_single_char_names)]
    pub fn solution_2(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let distance_threshold = distance_threshold as i16;
        let mut g = vec![10001i16; n * n]; // init the graph, with default remain distance capability
        for edge in edges {
            unsafe {
                let a = *edge.get_unchecked(0) as usize;
                let b = *edge.get_unchecked(1) as usize;
                let w = *edge.get_unchecked(2) as i16;
                *g.get_unchecked_mut(a + b * n) = w;
                *g.get_unchecked_mut(a * n + b) = w;
            }
        }

        let mut best_count = n;
        let mut best_city = 0;

        let mut todo = vec![];
        let mut todo2 = vec![];
        let mut cache = HashMap::new();

        (0..n).for_each(|i| {
            todo.clear();
            todo.push((i, distance_threshold));
            todo2.clear();
            cache.clear();
            cache.insert(i, distance_threshold);
            loop {
                for (j, remain) in &todo {
                    for (k, cost) in g[j * n..(j + 1) * n].iter().enumerate() {
                        let new_remain = remain - *cost;
                        if new_remain >= 0 {
                            let mut can_go = true;
                            if let Some(old_remain) = cache.get(&k) {
                                can_go = *old_remain < new_remain;
                            }
                            if can_go {
                                cache.insert(k, new_remain);
                                if cache.len() > best_count {
                                    // println!("not found a better city this time");
                                    return; // this node results in more cities
                                }
                                todo2.push((k, new_remain));
                            }
                        }
                    }
                }
                if todo2.is_empty() {
                    best_count = cache.len();
                    best_city = i;
                    // println!("City {} -> {:?}", i, best_count);
                    return;
                }
                mem::swap(&mut todo, &mut todo2);
                todo2.clear();
            }
        });
        best_city as i32
    }

    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        Solution::floyd_warshall_algorithm(n, edges, distance_threshold)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1334() {
        assert_eq!(
            Solution::find_the_city(
                4,
                vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
                4
            ),
            3
        );
        assert_eq!(
            Solution::find_the_city(
                5,
                vec![
                    vec![0, 1, 2],
                    vec![0, 4, 8],
                    vec![1, 2, 3],
                    vec![1, 4, 2],
                    vec![2, 3, 1],
                    vec![3, 4, 1]
                ],
                2
            ),
            0
        );
        assert_eq!(
            Solution::find_the_city(
                8,
                vec![
                    vec![3, 5, 9558],
                    vec![1, 2, 1079],
                    vec![1, 3, 8040],
                    vec![0, 1, 9258],
                    vec![4, 7, 7558],
                    vec![5, 6, 8196],
                    vec![3, 4, 7284],
                    vec![1, 5, 6327],
                    vec![0, 4, 5966],
                    vec![3, 6, 8575],
                    vec![2, 5, 8604],
                    vec![1, 7, 7782],
                    vec![4, 6, 2857],
                    vec![3, 7, 2336],
                    vec![0, 6, 6],
                    vec![5, 7, 2870],
                    vec![4, 5, 5055],
                    vec![0, 7, 2904],
                    vec![1, 6, 2458],
                    vec![0, 5, 3399],
                    vec![6, 7, 2202],
                    vec![0, 2, 3996],
                    vec![0, 3, 7495],
                    vec![1, 4, 2262],
                    vec![2, 6, 1390]
                ],
                7937
            ),
            7
        );
        assert_eq!(
            Solution::find_the_city(
                6,
                vec![
                    vec![0, 3, 5],
                    vec![2, 3, 7],
                    vec![0, 5, 2],
                    vec![0, 2, 5],
                    vec![1, 2, 6],
                    vec![1, 4, 7],
                    vec![3, 4, 4],
                    vec![2, 5, 5],
                    vec![1, 5, 8]
                ],
                8279
            ),
            5
        );
    }
}
