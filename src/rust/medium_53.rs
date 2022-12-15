use std::i32::MAX;

/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_sub_array_v1(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        for i in 0..nums.len() {
            let mut count = 0;
            for j in i..nums.len() {
                count += nums[j];
                result = count.max(result);
            }
        }
        result
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        let mut count = 0;
        for i in 0..nums.len() {
            count += nums[i];
            result = count.max(result);
            count = count.max(0);
        }
        result
    }
}
// @lc code=end

