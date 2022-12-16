use core::str;

/*
 * @lc app=leetcode id=452 lang=rust
 *
 * [452] Minimum Number of Arrows to Burst Balloons
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        if points.len() == 0 {
            return 0;
        }

        points.sort_by_key(|point| point[0]);
        let mut result = 1;
        for i in 1..points.len() {
            if points[i][0] > points[i - 1][1] {
                result += 1;
            } else {
                points[i][1] = points[i][1].min(points[i - 1][1]);
            }
        }
        result
    }
}
// @lc code=end
