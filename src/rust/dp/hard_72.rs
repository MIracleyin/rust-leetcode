/*
 * @lc app=leetcode id=72 lang=rust
 *
 * [72] Edit Distance
 */
struct Solution;
// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.chars().collect::<Vec<char>>();
        let word2 = word2.chars().collect::<Vec<char>>();
        let (w1, w2) = (word1.len(), word2.len());
        let mut dp = vec![vec![0; w2 + 1]; w1 + 1];
        // dp[i][j] 表示以下标i-1为结尾的字符串word1，和以下标j-1为结尾的字符串word2，最近编辑距离为dp[i][j]。
        for i in 0..=w1 {
            dp[i][0] = i as i32;
        }
        for j in 0..=w2 {
            dp[0][j] = j as i32;
        }
        for i in 1..=w1 {
            for j in 1..=w2 {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = min(dp[i - 1][j - 1], dp[i][j - 1].min(dp[i - 1][j])) + 1;
                }
            }
        }
        dp[w1][w2]
    }
}
// @lc code=end

