/*
 * @lc app=leetcode id=96 lang=rust
 *
 * [96] Unique Binary Search Trees
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        //  1到i为节点组成的二叉搜索树的个数为dp[i]
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dbg!(dp.clone());
        dp[n] as i32
    }
}
// @lc code=end
#[test]
fn it_works() {
    Solution::num_trees(5);
}

