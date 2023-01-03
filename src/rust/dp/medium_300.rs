/*
 * @lc app=leetcode id=300 lang=rust
 *
 * [300] Longest Increasing Subsequence
 */
struct Solution;
// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 1 {
            return len as i32;
        }
        // dp[i]表示i之前包括i的以nums[i]结尾的最长递增子序列的长度
        let mut dp = vec![1; len];
        let mut result = 0;
        for i in 1..len {
            for j in 0..i {
                if nums[i] > nums[j] { // next bigger
                    dp[i] = max(dp[i], dp[j] + 1);
                }
            }
            result = result.max(dp[i]);
        }
        result
    }
}
// @lc code=end

