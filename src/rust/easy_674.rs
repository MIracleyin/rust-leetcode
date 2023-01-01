/*
 * @lc app=leetcode id=674 lang=rust
 *
 * [674] Longest Continuous Increasing Subsequence
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut result = 1;
        let mut dp = vec![1; len];
        for i in 1..len {
            if nums[i] > nums[i-1] {
                dp[i] = dp[i - 1] + 1;
            }
            result = result.max(dp[i]);
        }
        result
    }
}
// @lc code=end

