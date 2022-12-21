/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn climb_stairs_v1(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }

    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        for j in 0..=n as usize {
            for i in 1..=2 {
                if j >= i {
                    dp[j] += dp[j - i];
                }
            }
        }
        dp[n as usize]
    }
}
// @lc code=end
