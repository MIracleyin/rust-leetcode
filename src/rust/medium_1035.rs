/*
 * @lc app=leetcode id=1035 lang=rust
 *
 * [1035] Uncrossed Lines
 */
struct Solution;
// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (l1, l2) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        for i in 1..=l1 {
            for j in 1..=l2 {
                if nums1[i - 1] == nums2[j - 1] {
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

