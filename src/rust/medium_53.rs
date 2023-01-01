/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */
struct Solution;
// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut dp = vec![0; len];
        dp[0] = nums[0];
        let mut result = dp[0];
        for i in 1..len {
            dp[i] = max(dp[i - 1] + nums[i], nums[i]);
            result = result.max(dp[i]);
        }
        result
    }
}
// @lc code=end

