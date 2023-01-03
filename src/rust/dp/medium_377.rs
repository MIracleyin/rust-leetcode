use std::vec;

/*
 * @lc app=leetcode id=377 lang=rust
 *
 * [377] Combination Sum IV
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];

        dp[0] = 1; // 0! = 1

        for j in 0..=target as usize{
            for i in 0..nums.len() {
                if j >= nums[i] as usize {
                    dp[j] += dp[j - nums[i] as usize];
                }
            }
        }

        dp[target as usize]
    }
}
// @lc code=end

