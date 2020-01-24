/**
 * [1263] Minimum Moves to Move a Box to Their Target Location
 *
 * Storekeeper is a game, in which the player pushes boxes around in a warehouse, trying to get them to target locations.
 * The game is represented by a grid of size n<font face="sans-serif, Arial, Verdana, Trebuchet MS">*</font>m, where each element is a wall, floor or a box.
 * Your task is move the box 'B' to the target position 'T' under the following rules:
 *
 * 	Player is represented by character 'S' and can move up, down, left, right in the grid if its a floor (empy cell).
 * 	Floor is represented by character '.' that means free cell to walk.
 * 	Wall is represented by character '#' that means obstacle  (impossible to walk there).
 * 	There is only one box 'B' and one target cell 'T' in the grid.
 * 	The box can be moved to an adjacent free cell by standing next to the box and then moving in the direction of the box. This is a push.
 * 	The player cannot walk through the box.
 *
 * Return the minimum number of pushes to move the box to the target. If there is no way to reach the target, return -1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/06/sample_1_1620.png" style="width: 520px; height: 386px;" />
 *
 * Input: grid = [["#","#","#","#","#","#"],
 *                ["#","T","#","#","#","#"],
 *                ["#",".",".","B",".","#"],
 *                ["#",".","#","#",".","#"],
 *                ["#",".",".",".","S","#"],
 *                ["#","#","#","#","#","#"]]
 * Output: 3
 * Explanation: We return only the number of times the box is pushed.
 * Example 2:
 *
 * Input: grid = [["#","#","#","#","#","#"],
 *                ["#","T","#","#","#","#"],
 *                ["#",".",".","B",".","#"],
 *                ["#","#","#","#",".","#"],
 *                ["#",".",".",".","S","#"],
 *                ["#","#","#","#","#","#"]]
 * Output: -1
 *
 * Example 3:
 *
 * Input: grid = [["#","#","#","#","#","#"],
 *                ["#","T",".",".","#","#"],
 *                ["#",".","#","B",".","#"],
 *                ["#",".",".",".",".","#"],
 *                ["#",".",".",".","S","#"],
 *                ["#","#","#","#","#","#"]]
 * Output: 5
 * Explanation:  push the box down, left, left, up and up.
 *
 * Example 4:
 *
 * Input: grid = [["#","#","#","#","#","#","#"],
 *                ["#","S","#",".","B","T","#"],
 *                ["#","#","#","#","#","#","#"]]
 * Output: -1
 *
 *  
 * Constraints:
 *
 * 	1 <= grid.length <= 20
 * 	1 <= grid[i].length <= 20
 * 	grid contains only characters '.', '#',  'S' , 'T', or 'B'.
 * 	There is only one character 'S', 'B' <font face="sans-serif, Arial, Verdana, Trebuchet MS">and </font>'T' in the grid.
 *
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashSet;

impl Solution {
    #[allow(clippy::many_single_char_names)]
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let mut g = vec![];
        let mut b = (0usize, 0usize); // box
        let mut t = (0usize, 0usize); // target
        let mut s = (0usize, 0usize); // start

        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.iter().cloned().enumerate() {
                match c {
                    'B' => {
                        b = (i, j);
                    }
                    'T' => {
                        t = (i, j);
                    }
                    'S' => {
                        s = (i, j);
                    }
                    _ => (),
                }
                g.push(c);
            }
        }

        // get the char of position "p" in grid "g"
        let pos = |g: &Vec<char>, p: (usize, usize)| -> char {
            if p.0 < row && p.1 < col {
                return g[p.0 * col + p.1];
            }
            '#'
        };

        // test whether p1 can reach p2
        let can_go = |g: &Vec<char>, p1: (usize, usize), p2: (usize, usize)| -> bool {
            let c = g[p2.0 * col + p2.1];
            if c == '#' || c == 'B' {
                return false;
            }
            if p1 == p2 {
                return true;
            }
            let mut mem = HashSet::new();
            mem.insert(p1);
            let mut todo = vec![p1];
            while let Some((i1, j1)) = todo.pop() {
                for &p in &[
                    (i1 + 1, j1),
                    (i1, j1 + 1),
                    (i1.wrapping_sub(1), j1),
                    (i1, j1.wrapping_sub(1)),
                ] {
                    if p == p2 {
                        return true;
                    }
                    let c = pos(&g, p);
                    if c != '#' && c != 'B' && !mem.contains(&p) {
                        mem.insert(p);
                        todo.push(p);
                    }
                }
            }
            false
        };

        // BFS every posibility
        let mut processed = HashSet::new();
        let mut todo = vec![(g, b, s)];
        let mut step = 0;
        while !todo.is_empty() {
            let mut todo2 = vec![];
            step += 1;
            for (g, b, s) in todo {
                for &nxt_b in &[
                    (b.0 + 1, b.1),
                    (b.0, b.1 + 1),
                    (b.0 - 1, b.1),
                    (b.0, b.1 - 1),
                ] {
                    if pos(&g, nxt_b) == '#' {
                        continue;
                    }
                    let diff = (nxt_b.0.wrapping_sub(b.0), nxt_b.1.wrapping_sub(b.1));
                    let back = (b.0.wrapping_sub(diff.0), b.1.wrapping_sub(diff.1));
                    if back.0 >= row || back.1 >= col || back == t {
                        continue;
                    }
                    if can_go(&g, s, back) {
                        if nxt_b == t {
                            return step;
                        }
                        let mut gg = g.clone();
                        gg[s.0 * col + s.1] = '.';
                        gg[b.0 * col + b.1] = 'S';
                        gg[nxt_b.0 * col + nxt_b.1] = 'B';
                        if !processed.contains(&(b, diff)) {
                            processed.insert((b, diff));
                            todo2.push((gg, nxt_b, b));
                        }
                    } // end if can_go
                }
            }
            todo = todo2;
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1263() {
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '#', '#', '#', '#', '#'],
                vec!['#', 'T', '#', '#', '#', '#'],
                vec!['#', '.', '.', 'B', '.', '#'],
                vec!['#', '.', '#', '#', '.', '#'],
                vec!['#', '.', '.', '.', 'S', '#'],
                vec!['#', '#', '#', '#', '#', '#']
            ]),
            3
        );
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '#', '#', '#', '#', '#'],
                vec!['#', 'T', '#', '#', '#', '#'],
                vec!['#', '.', '.', 'B', '.', '#'],
                vec!['#', '#', '#', '#', '.', '#'],
                vec!['#', '.', '.', '.', 'S', '#'],
                vec!['#', '#', '#', '#', '#', '#']
            ]),
            -1
        );
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '#', '#', '#', '#', '#'],
                vec!['#', 'T', '.', '.', '#', '#'],
                vec!['#', '.', '#', 'B', '.', '#'],
                vec!['#', '.', '.', '.', '.', '#'],
                vec!['#', '.', '.', '.', 'S', '#'],
                vec!['#', '#', '#', '#', '#', '#']
            ]),
            5
        );
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '#', '#', '#', '#', '#', '#'],
                vec!['#', 'S', '#', '.', 'B', 'T', '#'],
                vec!['#', '#', '#', '#', '#', '#', '#']
            ]),
            -1
        );
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '.', '.', '#', 'T', '#', '#', '#', '#'],
                vec!['#', '.', '.', '#', '.', '#', '.', '.', '#'],
                vec!['#', '.', '.', '#', '.', '#', 'B', '.', '#'],
                vec!['#', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.', 'S', '#'],
                vec!['#', '.', '.', '#', '.', '#', '#', '#', '#']
            ]),
            8
        );
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '.', '.', '#', '#', '#', '#', '#'],
                vec!['#', '.', '.', 'T', '#', '.', '.', '#'],
                vec!['#', '.', '.', '.', '#', 'B', '.', '#'],
                vec!['#', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '.', '.', '.', '#', '.', 'S', '#'],
                vec!['#', '.', '.', '#', '#', '#', '#', '#']
            ]),
            7
        );
    }
}
