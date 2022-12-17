/*
 * @lc app=leetcode id=746 lang=rust
 *
 * [746] Min Cost Climbing Stairs
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_cost_climbing_stairs_v1(cost: Vec<i32>) -> i32 {
        // dp 到达第 i 阶梯所花费的最少体力为 dp[i]
        let mut dp = vec![0; cost.len() + 1];
        // dp[i] = d[i-1] + cost[i-1]
        // dp[i] = d[i-2] + cost[i-2]
        // min(a, b)
        // dp[0] = 0 dp[1] = 0
        dp[0] = 0;
        dp[1] = 0;
        for i in 2..=cost.len() {
            dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
        }
        dp[cost.len()]
    }
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // dp 到达第 i 阶梯所花费的最少体力为 dp[i]
        let mut dp = vec![0; 2];
        // dp[i] = d[i-1] + cost[i-1]
        // dp[i] = d[i-2] + cost[i-2]
        // min(a, b)
        // dp[0] = 0 dp[1] = 0
        dp[0] = 0;
        dp[1] = 0;
        for i in 2..=cost.len() {
            let sum = (dp[1] + cost[i - 1]).min(dp[0] + cost[i - 2]);
            dp[0] = dp[1];
            dp[1] = sum;
        }
        dbg!(dp.clone());
        dp[1]
    }
}
// @lc code=end
#[test]
fn it_works() {
    let cost = vec![10,15,20];
    Solution::min_cost_climbing_stairs(cost);
}