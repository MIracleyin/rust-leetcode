/*
 * @lc app=leetcode id=213 lang=rust
 *
 * [213] House Robber II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        if len == 1 {
            return nums[0];
        }
        let res1 = Self::rob_range(&nums, 0, len - 2);
        let res2 = Self::rob_range(&nums, 1, len - 1);
        res1.max(res2)
    }

    fn rob_range(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        if end == start {
            return nums[start];
        }
        let mut dp = vec![0; nums.len()];
        dp[start] = nums[start]; // 偷第一家
        dp[start + 1] = nums[start].max(nums[start + 1]);
        for i in start + 2..=end {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }

        dp[end]
    }
}
// @lc code=end
#[test]
fn it_works() {
    let nums = vec![1,2,3,1];
    Solution::rob(nums);
}
