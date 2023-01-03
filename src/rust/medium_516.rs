/*
 * @lc app=leetcode id=516 lang=rust
 *
 * [516] Longest Palindromic Subsequence
 */
struct Solution;
// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        // dp[i][j]：字符串s在[i, j]范围内最长的回文子序列的长度为dp[i][j]。
        let len = s.len();
        let mut dp = vec![vec![0; len]; len];
        let mut result = 0;
        for i in 0..len {
            dp[i][i] = 1;
        }
        for i in (0..len).rev() {
            for j in i + 1..len {
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2
                } else {
                    dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[0][len - 1]
    }
}
// @lc code=end
