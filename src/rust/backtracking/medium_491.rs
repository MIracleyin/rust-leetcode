
/*
 * @lc app=leetcode id=491 lang=rust
 *
 * [491] Increasing Subsequences
 */
struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        Self::backtracking(&mut result, &mut path, &nums, 0);
        result
    }

    pub fn backtracking(result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, nums: &Vec<i32>, start_idx: usize) {
        if path.len() > 1 {
            result.push(path.to_vec());
            // 注意这里不要加return，因为要取树上的所有节点
            // return;
        }
        let mut set: HashSet<i32> = HashSet::new();
        for i in start_idx..nums.len() {
            if (!path.is_empty() && nums[i] < *path.last().unwrap()) || set.contains(&nums[i]) {
                continue;
            }
            set.insert(nums[i]);
            path.push(nums[i]);
            Self::backtracking(result, path, nums, i + 1);
            path.pop();
        }
    }
}
// @lc code=end

