use std::vec;

/*
 * @lc app=leetcode id=518 lang=rust
 *
 * [518] Coin Change II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn change_v1(amount: i32, coins: Vec<i32>) -> i32 {
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
        dp[amount as usize]
    }
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // 凑足总额为j所需钱币的最少个数为dp[j]
        let amount  = amount as usize;
        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0;
        // for j in 1..=amount as usize {
        //     for i in 0..coins.len() {
        //         if j >= coins[i] as usize && dp[j - coins[i] as usize] != i32::MAX {
        //             dp[j] = dp[j].min(dp[j - coins[i] as usize] + 1);
        //         }
        //     }
        // }
        for i in 0..coins.len() {
            for j in coins[i] as usize..=amount {
                if dp[j - coins[i] as usize] != i32::MAX {
                    dp[j] = dp[j].min(dp[j - coins[i] as usize] + 1);
                }
            }
        }
        dbg!(dp.clone());
        if dp[amount] == i32::MAX { -1 } else { dp[amount] }
    }
}
// @lc code=end
#[test]
fn it_works() {
    let amount = 5;
    let coins = vec![1,2,5];
    Solution::change(amount, coins);
}