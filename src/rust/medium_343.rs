use std::cmp;

/*
 * @lc app=leetcode id=343 lang=rust
 *
 * [343] Integer Break
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        // dp[i] num i 最大乘积
        let n = n as usize;
        let mut dp = vec![0; n + 1]; // dp[2] = 1
        dp[2] = 1;
        // dp[i] = j * (i - j)
        // d[i] = j * dp[i - j]
        // dp[i] = max(dp[i], j*(i-j), j*dp[i-j])
        for i in 3..=n {
            for j in 1..i / 2 {
                dp[i] = dp[i].max(j * (i - j)).max(j * dp[i - j]);
            }
        }
        dp[n] as i32
    }
}
// @lc code=end
#[test]
fn it_works() {
    Solution::integer_break(10);
}
