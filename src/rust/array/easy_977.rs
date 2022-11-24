/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn sorted_squares_v1(nums: Vec<i32>) -> Vec<i32> {
        let mut squart_nums = Vec::new();
        for n in nums {
            squart_nums.push(n * n);
        }
        squart_nums.sort();
        squart_nums

    }

    pub fn sorted_squares_v2(nums: Vec<i32>) -> Vec<i32> {
        let mut k = nums.len();
        let mut res_nums = vec![0; k];

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            if nums[l] * nums[l] < nums[r] * nums[r]  {
                res_nums[k - 1] = nums[r] * nums[r];
                r -= 1;
            } else {
                res_nums[k - 1] = nums[l] * nums[l];
                l += 1;
            }
            k -= 1;
        }
        res_nums

    }
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut n = nums.len();
        let mut res_nums = vec![0; n];

        let (mut l, mut r, mut k) = (0, n-1, n-1);

        while l <= r {
            if nums[l] * nums[l] < nums[r] * nums[r]  {
                res_nums[k] = nums[r] * nums[r];
                r -= 1;
            } else {
                res_nums[k] = nums[l] * nums[l];
                l += 1;
            }
            k -= 1;
        }

        res_nums

    }
}
// @lc code=end

