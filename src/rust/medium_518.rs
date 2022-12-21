use std::vec;

/*
 * @lc app=leetcode id=518 lang=rust
 *
 * [518] Coin Change II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // dp[j]：凑成总金额j的货币组合数为dp[j]
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;
        // for j in 0..=amount as usize { // 排列
        //     for i in 0..coins.len() {
        //         if j >= coins[i] as usize {
        //             dp[j] += dp[j - coins[i] as usize];
        //         }
        //     }
        // }
        for i in 0..coins.len() { // 组合
            for j in coins[i] as usize..=amount as usize {
                dp[j] += dp[j - coins[i] as usize];
            }
        }
        dbg!(dp.clone());
        dp[amount as usize]
    }
}
// @lc code=end
#[test]
fn it_works() {
    let amount = 5;
    let coins = vec![1,2,5];
    Solution::change(amount, coins);
}