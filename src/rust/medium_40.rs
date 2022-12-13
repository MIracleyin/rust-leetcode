/*
 * @lc app=leetcode id=40 lang=rust
 *
 * [40] Combination Sum II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        let mut used: Vec<bool> = vec![false; candidates.len()];
        let mut candidates = candidates;
        candidates.sort();
        Solution::backtracking(&candidates, target, &mut result, &mut path, 0, 0, &mut used);
        result
    }

    pub fn backtracking(
        candidates: &Vec<i32>,
        target: i32,
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        mut sum: i32,
        start_idx: usize,
        used: &mut Vec<bool>,
    ) {
        if sum == target {
            result.push(path.to_vec());
            return;
        }

        for i in start_idx..candidates.len() {
            if sum + candidates[i] <= target {
                if i > 0 && candidates[i] == candidates[i - 1] && used[i - 1] == false {
                    continue;
                }
                sum += candidates[i];
                path.push(candidates[i]);
                used[i] = true;
                Self::backtracking(candidates, target, result, path, sum, i + 1, used);
                used[i] = false;
                sum -= candidates[i];
                path.pop();
            }
        }
    }
}
// @lc code=end
