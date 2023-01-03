/*
 * @lc app=leetcode id=309 lang=rust
 *
 * [309] Best Time to Buy and Sell Stock with Cooldown
 */
struct Solution;
// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; 4]; len];
        // 状态一：持有股票状态（今天买入股票，或者是之前就买入了股票然后没有操作，一直持有）dp[i][0] = dp[i - 1][0]
        // 不持有股票状态，这里就有两种卖出股票状态
        // 状态二：保持卖出股票的状态（
        // 两天前就卖出了股票，度过一天冷冻期。
        // dp[i - 1][3] - prices[i]
        // 或者是前一天就是卖出股票状态，一直没操作）
        // dp[i - 1][1] - prices[i]
        // 状态三：今天卖出股票
        // dp[i - 1][0] + prices[i];
        // 状态四：今天为冷冻期状态，但冷冻期状态不可持续，只有一天！
        // dp[i][3] = dp[i - 1][2];
        dp[0][0] = -prices[0];
        for i in 1..len {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][3] - prices[i]).max(dp[i - 1][1] - prices[i]);
            dp[i][1] = max(dp[i - 1][1], dp[i - 1][3]);
            dp[i][2] = dp[i - 1][0] + prices[i];
            dp[i][3] = dp[i - 1][2];
        }
        max(dp[len - 1][1], dp[len - 1][2]).max(dp[len - 1][3])
    }
}
// @lc code=end
