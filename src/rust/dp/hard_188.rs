/*
 * @lc app=leetcode id=188 lang=rust
 *
 * [188] Best Time to Buy and Sell Stock IV
 */
struct Solution;
// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let len = prices.len();
        let mut dp = vec![vec![0; 2 * k + 1]; len];
        for j in (1..2 * k).step_by(2) {
            dp[0][j] = -prices[0];
        }
        for i in 1..len {
            for j in (0..2 * k - 1).step_by(2) {
                dp[i][j + 1] = max(dp[i - 1][j + 1], dp[i - 1][j] - prices[i]);
                dp[i][j + 2] = max(dp[i - 1][j + 2], dp[i - 1][j + 1] + prices[i]);
            }
        }
        dp[len - 1][2 * k]
    }
}
// @lc code=end
