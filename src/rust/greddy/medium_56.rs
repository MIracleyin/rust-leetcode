/*
 * @lc app=leetcode id=56 lang=rust
 *
 * [56] Merge Intervals
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by_key(|interval| interval[0]);
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        result.push(intervals[0].clone());
        for i in 1..intervals.len() {
            if result.last().unwrap()[1] >= intervals[i][0] {
                result.last_mut().unwrap()[1] = intervals[i][1].max(result.last().unwrap()[1]);
            } else {
                result.push(intervals[i].clone());
            }
        }
        result
    }
}
// @lc code=end
