/*
 * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        Self::backtracking(&mut result, &mut path, &nums, 0);
        result
    }

    pub fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        start_idx: usize,
    ) {
        result.push(path.clone()); // push all path
        let len = nums.len();
        if start_idx >= len {
            return;
        }
        for i in start_idx..len {
            path.push(nums[i]);
            Self::backtracking(result, path, nums, i + 1);
            path.pop();
        }
    }
}
// @lc code=end
