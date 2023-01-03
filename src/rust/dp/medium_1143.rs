/*
 * @lc app=leetcode id=1143 lang=rust
 *
 * [1143] Longest Common Subsequence
 */
struct Solution;
// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.chars().collect::<Vec<char>>();
        let text2 = text2.chars().collect::<Vec<char>>();
        let (l1, l2) = (text1.len(), text2.len());
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        for i in 1..=l1 {
            for j in 1..=l2 {
                if text1[i - 1] == text2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        // dbg!(dp.clone());
        dp[l1][l2]
    }
}
// @lc code=end
#[test]
fn it_works() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    Solution::longest_common_subsequence(text1, text2);
}