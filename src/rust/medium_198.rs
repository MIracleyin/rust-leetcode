use core::num;

/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // dp[i]：考虑下标i（包括i）以内的房屋，最多可以偷窃的金额为dp[i]。
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        if len == 1 {
            return nums[0];
        }
        let mut dp = vec![0; nums.len()];
        // dp[0] dp[1]
        dp[0] = nums[0];
        dp[1] = nums[1].max(nums[0]);
        // dp[i] = max(dp[i - 2] + nums[i], dp[i - 1]);
        for i in 2..nums.len() {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }
        dp[nums.len() - 1]
    }
}
// @lc code=end
