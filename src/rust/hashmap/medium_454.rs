
/*
 * @lc app=leetcode id=454 lang=rust
 *
 * [454] 4Sum II
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for n1 in &nums1 {
            for n2 in &nums2 {
                *map.entry(n1 + n2).or_insert(0) += 1;
            }
        }

        let mut cnt = 0;

        for n3 in &nums3 {
            for n4 in &nums4 {
                let target = - (n3 + n4);
                cnt += map.get(&target).unwrap_or(&0);
            }
        }
        cnt
    }
}
// @lc code=end

