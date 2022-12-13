use std::collections::{HashMap, HashSet};

/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        let mut used: Vec<bool> = vec![false; nums.len()];
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
            if used[i] == true {
                continue;
            }
            used[i] = true;
            path.push(nums[i]);
            Self::backtracking(result, path, nums, used);
            path.pop();
            used[i] = false;
        }
    }
}
// @lc code=end
