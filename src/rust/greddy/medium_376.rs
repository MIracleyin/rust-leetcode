use core::num;

/*
 * @lc app=leetcode id=376 lang=rust
 *
 * [376] Wiggle Subsequence
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut cur_diff = 0;
        let mut pre_diff = 0;
        let mut result = 1;
        for i in 0..nums.len() - 1 {
            cur_diff = nums[i + 1] - nums[i];
            if (cur_diff > 0 && pre_diff <= 0) || (cur_diff < 0 && pre_diff >= 0) {
                result += 1;
                pre_diff = cur_diff;
            }
        }
        result
    }
}
// @lc code=end
#[test]
fn it_works() {
    let nums = vec![1, 7, 4, 9, 2, 5];
    Solution::wiggle_max_length(nums);
}
