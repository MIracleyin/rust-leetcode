/*
 * @lc app=leetcode id=714 lang=rust
 *
 * [714] Best Time to Buy and Sell Stock with Transaction Fee
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut result = 0;
        let mut min_price = prices[0]; // 

        for i in 1..prices.len() {
            if prices[i] < min_price {
                min_price = prices[i];
            }
            // if prices[i] >= min_price && prices[i] <= min_price + fee {
            //     continue;
            // }
            if prices[i] > min_price + fee {
                result += prices[i] - min_price - fee;
                min_price = prices[i] - fee;
            }
        }

        result
    }
}
// @lc code=end

