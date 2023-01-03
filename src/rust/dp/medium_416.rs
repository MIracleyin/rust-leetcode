/*
 * @lc app=leetcode id=416 lang=rust
 *
 * [416] Partition Equal Subset Sum
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        // size = nums.sum()/2
        let sum: i32 = nums.iter().sum();
        let target = sum as usize / 2;
        if sum % 2 == 1 {
            return false;
        }

        // dp[j] 表示： 容量为j的背包，所背的物品价值可以最大为dp[j]
        // dp[j]表示 背包总容量（所能装的总重量）是j，放进物品后，背的最大重量为dp[j]。
        let mut dp = vec![0; target + 1];


        // dp[j] = max(dp[j], dp[j - nums[i]] + nums[i]);

        // dp[j]的定义来看，首先dp[0]一定是0。
        for i in 0..nums.len() {
            for j in (nums[i] as usize..=target).rev() {
                dp[j] = dp[j].max(dp[j - nums[i] as usize] + nums[i]);
            }
        }
        if dp[target] == target as i32 {
            true
        } else {
            false
        }
        
    }
}
// @lc code=end
#[test]
fn it_works() {
    let nums = vec![1,5,11,5];
    Solution::can_partition(nums);
}