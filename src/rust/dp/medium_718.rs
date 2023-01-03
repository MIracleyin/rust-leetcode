/*
 * @lc app=leetcode id=718 lang=rust
 *
 * [718] Maximum Length of Repeated Subarray
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (l1, l2) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        let mut result = 0;
        for i in 1..=l1 {
            for j in 1..=l2 {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                }
                result = result.max(dp[i][j]);
            }
        }
        result
    }
}
// @lc code=end
