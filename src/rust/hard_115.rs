/*
 * @lc app=leetcode id=115 lang=rust
 *
 * [115] Distinct Subsequences
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();
        let (s_len, t_len) = (s.len(), t.len());
        let mut dp = vec![vec![0; t_len + 1]; s_len + 1];
        // dp[i][j]：以i-1为结尾的s子序列中出现以j-1为结尾的t的个数为dp[i][j]。
        for i in 0..s_len {
            dp[i][0] = 1;
        }
        for j in 1..t_len {
            dp[0][j] = 0;
        }
        for i in 1..=s_len {
            for j in 1..=t_len {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        dp[s_len][t_len]
    }
}
// @lc code=end

