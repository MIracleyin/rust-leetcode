use std::process::id;

/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut cover: i32 = 0;
        if nums.len() == 1 {
            return true;
        }
        let mut idx: i32 = 0;
        while idx <= cover {
            cover = cover.max(nums[idx as usize] + idx);
            if cover >= nums.len() as i32 - 1 {
                return true;
            }
            idx += 1;
        }
        false
    }
}
// @lc code=end

