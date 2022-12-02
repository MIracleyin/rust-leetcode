/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 * [349] 两个数组的交集
 */
struct Solution;

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn intersection_v1(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // 1, 2, 2, 1
        // set_nums1 -> 1, 2
        // for i in 2,2
        // records
        // 2
        let set1: HashSet<i32> = nums1.iter().map(|x| *x).collect();

        let mut res: HashSet<i32> = HashSet::new();

        for n in nums2.iter() {
            if set1.contains(n) {
                res.insert(*n);
            }
        }

        let res: Vec<i32> = res.iter().map(|x| *x).collect();
        res
    }

    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .into_iter()
            .collect::<HashSet<_>>()
            .intersection(&nums2.into_iter().collect::<HashSet<_>>())
            .copied()
            .collect()
    }
}
// @lc code=end
