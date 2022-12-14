/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        Self::backtracking(&mut result, &mut path, &candidates, target, 0, 0);
        result
    }
    pub fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        candidates: &Vec<i32>,
        target: i32,
        mut sum: i32,
        start_idx: usize,
    ) {
        if sum == target {
            result.push(path.to_vec());
            return;
        }
        for i in start_idx..candidates.len() {
            if sum + candidates[i] <= target {
                sum += candidates[i];
                path.push(candidates[i]);
                Self::backtracking(result, path, candidates, target, sum, i);
                sum -= candidates[i];
                path.pop();
            }
        }
    }
}
// @lc code=end
