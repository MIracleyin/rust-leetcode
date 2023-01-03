/*
 * @lc app=leetcode id=122 lang=rust
 *
 * [122] Best Time to Buy and Sell Stock II
 */
struct Solution;
// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut dp = vec![vec![0; 2]; len];
        dp[0][0] = -prices[0];
        dp[0][1] = 0;
        for i in 1..len {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] - prices[i]);
            dp[i][1] = max(dp[i - 1][1], dp[i - 1][0] + prices[i]);
        }
        dp[len - 1][1]
    }
}
// @lc code=end
