/*
 * @lc app=leetcode id=392 lang=rust
 *
 * [392] Is Subsequence
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();
        let (s_len, t_len) = (s.len(), t.len());
        // dp[i][j] 表示以下标i-1为结尾的字符串s，和以下标j-1为结尾的字符串t，相同子序列的长度为dp[i][j]。
        let mut dp = vec![vec![0; t_len + 1]; s_len + 1];
        for i in 1..=s_len {
            for j in 1..=t_len {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i][j - 1];
                }
            }
        }
        if dp[s_len][t_len] == s_len {
            return true;
        } else {
            return false;
        }
    }
}
// @lc code=end
