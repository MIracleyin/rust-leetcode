use core::num;

/*
 * @lc app=leetcode id=1005 lang=rust
 *
 * [1005] Maximize Sum Of Array After K Negations
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut nums = nums;
        let mut k = k;
        nums.sort_by(|a, b| b.abs().cmp(&a.abs()));
        for i in 0..len {
            if nums[i] < 0 && k > 0 {
                nums[i] *= -1;
                k -= 1;
            }
        }
        if k % 2 == 1 {
            nums[len - 1] *= -1;
        }
        nums.iter().sum()
    }
}
// @lc code=end

