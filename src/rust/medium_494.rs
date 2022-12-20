use core::num;

/*
 * @lc app=leetcode id=494 lang=rust
 *
 * [494] Target Sum
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let sum = nums.iter().sum();
        if target > sum {
            return 0;
        }
        if (target + sum) % 2 != 0 {
            return 0;
        }

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        let bag_size = (target + sum) / 2;
        Self::backtracking(&mut result, &mut path, &nums, bag_size, 0, 0);
        result.len() as i32
    }
    pub fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        target: i32,
        mut sum: i32,
        start_idx: usize,
    ) {
        if sum == target {
            result.push(path.to_vec());
        }

        for i in start_idx..nums.len() {
            if sum + nums[i] <= target {
                sum += nums[i];
                path.push(nums[i]);
                Self::backtracking(result, path, nums, target, sum, i+1);
                path.pop();
                sum -= nums[i];
            }
        }
    }
}
// @lc code=end
