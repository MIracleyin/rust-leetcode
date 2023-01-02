/*
 * @lc app=leetcode id=647 lang=rust
 *
 * [647] Palindromic Substrings
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let len = s.len();
        let mut dp = vec![vec![false; len]; len];
        let mut result = 0;
        for i in (0..len).rev() {
            for j in i..len {
                if s[i] == s[j] {
                    if j - i <= 1 {
                        result += 1;
                        dp[i][j] = true;
                    } else if dp[i + 1][j - 1] {
                        result += 1;
                        dp[i][j] = true;
                    }
                }
            }
        }
        result
    }
}
// @lc code=end

