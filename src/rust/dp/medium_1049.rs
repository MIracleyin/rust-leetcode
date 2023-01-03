/*
 * @lc app=leetcode id=1049 lang=rust
 *
 * [1049] Last Stone Weight II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();
        let target = sum as usize / 2;

        let mut dp = vec![0; target + 1];
        for i in 0..stones.len() {
            for j in (stones[i] as usize..=target).rev() {
                dp[j] = dp[j].max(dp[j - stones[i] as usize] + stones[i]);
            }
        }

        sum - dp[target] * 2
    }
}
// @lc code=end

