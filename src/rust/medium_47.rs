use std::result;

/*
 * @lc app=leetcode id=47 lang=rust
 *
 * [47] Permutations II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        let mut nums = nums;
        let mut used: Vec<bool> = vec![false; nums.len()];
        nums.sort();
        Self::backtracking(&mut result, &mut path, &nums, &mut used);
        result
    }

    pub fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
    ) {
        if path.len() == nums.len() {
            result.push(path.to_vec());
            return;
        }
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] && used[i - 1] == false {
                continue;
            }
            if used[i] == false {
                used[i] = true;
                path.push(nums[i]);
                Self::backtracking(result, path, nums, used);
                path.pop();
                used[i] = false;
            }
        }
    }
}
// @lc code=end
