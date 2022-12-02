/*
 * @lc app=leetcode id=18 lang=rust
 *
 * [18] 4Sum
 */
struct Solution;

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            // to something
            if nums[i] > target && (nums[i] >= 0 || target >= 0) {
                break;
            }
            
            // deduplicate
            if i > 0 && (nums[i] == nums[i-1]) {
                continue;
            }

            for j in (i + 1)..nums.len() {
                // to something
                if nums[i] + nums[j] > target && (nums[i] + nums[j] >= 0 || target >= 0){
                    break;
                }

                // deduplicate
                if j > i + 1 && (nums[j] == nums[j-1]) {
                    continue;
                }

                let (mut l, mut r) = (j + 1, nums.len() - 1);
                while l < r {
                    match (nums[i] + nums[j] + nums[l] + nums[r]).cmp(&target) {
                        Ordering::Greater => {
                            r -= 1;
                        }
                        Ordering::Less => {
                            l += 1;
                        }
                        Ordering::Equal => {
                            res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                            while l < r && nums[r] == nums[r - 1] {
                                r -= 1;
                            }
                            while l < r && nums[l] == nums[l + 1] {
                                l += 1;
                            }
                            r -= 1;
                            l += 1;
                        }
                    }
                }
            }
        }

        res
    }
}
// @lc code=end
