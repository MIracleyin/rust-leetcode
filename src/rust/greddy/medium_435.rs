/*
 * @lc app=leetcode id=435 lang=rust
 *
 * [435] Non-overlapping Intervals
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 0 {
            return 0;
        }
        let mut intervals = intervals;
        intervals.sort_by_key(|r_side| r_side[1]);
        let mut result = 1;
        let mut end = intervals[0][1];
        for i in 1..intervals.len() {
            if end <= intervals[i][0] {
                end = intervals[i][1];
                result += 1
            }
        }
        intervals.len() as i32 - result
    }
}
// @lc code=end
#[test]
fn it_work() {
    let mut intervals = vec! [ vec![1,2], vec![2,3], vec![3,4], vec![1,3] ];
    Solution::erase_overlap_intervals(intervals);
}
