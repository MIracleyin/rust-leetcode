/*
 * @lc app=leetcode id=90 lang=rust
 *
 * [90] Subsets II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        let mut used: Vec<bool> = vec![false; nums.len()];
        let mut nums = nums;
        nums.sort();
        Self::backtracking(&mut result, &mut path, &nums, 0, &mut used);
        result
    }

    pub fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        start_idx: usize,
        used: &mut Vec<bool>,
    ) {
        result.push(path.to_vec());

        for i in start_idx..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] && used[i - 1] == false {
                continue;
            }
            path.push(nums[i]);
            used[i] = true;
            Self::backtracking(result, path, nums, i + 1, used);
            used[i] = false;
            path.pop();
        }
    }
}
// @lc code=end
