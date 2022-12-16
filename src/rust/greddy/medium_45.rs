/*
 * @lc app=leetcode id=45 lang=rust
 *
 * [45] Jump Game II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn jump_v1(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let (mut cur_dis, mut next_dis) = (0, 0);
        let mut res = 0;
        for i in 0..nums.len() {
            next_dis = next_dis.max(nums[i] + i as i32);
            if i == cur_dis as usize{
                if cur_dis as usize != nums.len() - 1 {
                    res += 1;
                    cur_dis = next_dis;
                    if cur_dis as usize >= nums.len() - 1 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        res
    }

    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut cur_dis, mut next_dis) = (0, 0);
        let mut res = 0;
        for i in 0..nums.len() - 1 {
            next_dis = next_dis.max(nums[i] + i as i32);
            if i == cur_dis as usize{
                if cur_dis as usize != nums.len() - 1 {
                    cur_dis = next_dis;
                    res += 1;
                }
            }
        }
        res
    }
}
// @lc code=end

