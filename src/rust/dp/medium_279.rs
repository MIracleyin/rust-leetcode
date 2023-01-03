/*
 * @lc app=leetcode id=279 lang=rust
 *
 * [279] Perfect Squares
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        // dp[j]：和为j的完全平方数的最少数量为dp[j]
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        // for i in 0..n {
        //     for j in 1..=i {
        //         dp[j] = dp[j].min(dp[j - i * i] + 1);
        //     }
        // }
        let items = (1..n as i32 + 1)
            .filter(|i| i * i <= n as i32)
            .map(|i| i * i)
            .collect::<Vec<i32>>();
        for j in 0..=n {
            for i in items.iter() {
                if j >= *i as usize {
                    dp[j] = dp[j].min(dp[j - *i as usize] + 1);
                }
            }
        }
        dbg!(dp.clone());
        dp[n]
    }
}
// @lc code=end
#[test]
fn it_works() {
    Solution::num_squares(5);
}
