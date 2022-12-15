/*
 * @lc app=leetcode id=122 lang=rust
 *
 * [122] Best Time to Buy and Sell Stock II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 1..prices.len() {
            result += 0.max(prices[i] - prices[i - 1]);
        }
        result
    }
}
// @lc code=end
