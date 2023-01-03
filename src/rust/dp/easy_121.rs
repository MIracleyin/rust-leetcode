/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */
struct Solution;
// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..prices.len() {
            for j in i..prices.len() {
                result = max(result, prices[j] - prices[i]);
            }
        }
        result
    }
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut low = i32::MAX;
        let mut result = 0;
        for i in 0..prices.len() {
            low = min(low, prices[i]);
            result = max(result, prices[i] - low);
        }
        result
    }
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len == 0 {
            return 0;
        }
        // dp[i][0] 表示第i天持有股票所得最多现金
        // dp[i][1] 表示第i天不持有股票所得最多现金
        let mut dp = vec![vec![0; 2]; len];
        dp[0][0] -= prices[0];
        dp[0][1] = 0;
        for i in 1..len {
            dp[i % 2][0] = max(dp[(i - 1) % 2][0], -prices[i]);
            dp[i % 2][1] = max(dp[(i - 1) % 2][1], prices[i] + dp[(i - 1) % 2][0]);
        }
        dp[(len - 1) % 2][1]
    }
    pub fn max_profit_v3(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len == 0 {
            return 0;
        }
        // dp[i][0] 表示第i天持有股票所得最多现金
        // dp[i][1] 表示第i天不持有股票所得最多现金
        let mut dp = vec![vec![0; 2]; len];
        dp[0][0] -= prices[0];
        dp[0][1] = 0;
        for i in 1..len {
            dp[i][0] = max(dp[i - 1][0], -prices[i]);
            dp[i][1] = max(dp[i - 1][1], prices[i] + dp[i - 1][0]);
        }
        dp[len - 1][1]
    }
}
// @lc code=end
