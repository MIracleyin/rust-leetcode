use core::num;

/*
 * @lc app=leetcode id=494 lang=rust
 *
 * [494] Target Sum
 */
struct Solution;
// @lc code=start
impl Solution {

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
                Self::backtracking(result, path, nums, target, sum, i + 1);
                path.pop();
                sum -= nums[i];
            }
        }
    }
    pub fn find_target_sum_ways_v1(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let sum: i32 = nums.iter().sum();
        if target > sum.abs() {
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

    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let sum: i32 = nums.iter().sum();
        if target.abs() > sum {
            return 0;
        }
        if (target + sum) % 2 == 1 { // target + sum must can diver 2
            return 0;
        }
        // dp[j] 表示：填满j（包括j）这么大容积的包，有dp[j]种方法
        let bag_size = (target + sum) as usize >> 1;
        let mut dp: Vec<i32> = vec![0; bag_size as usize + 1];
        // dp[j] += dp[j - nums[i]] 填满 0 尺寸的背包 有 1 种方法
        dp[0] = 1;
        for i in 0..nums.len() {
            for j in (nums[i] as usize..=bag_size).rev() {
                dp[j] += dp[j - nums[i] as usize];
            }
        }
        dp[bag_size]
    }
}
// @lc code=end
#[test]
fn it_works() {
    let nums = vec![1,1,1,1,1];
    let target = -3;
    Solution::find_target_sum_ways(nums, target);
}
