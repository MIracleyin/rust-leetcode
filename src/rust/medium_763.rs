

/*
 * @lc app=leetcode id=763 lang=rust
 *
 * [763] Partition Labels
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut map = HashMap::new();
        let s = s.chars().collect::<Vec<char>>();
        for (i, c) in s.iter().enumerate() {
            map.insert(c, i);
        }
        let mut result: Vec<i32> = Vec::new();
        let mut left = 0;
        let mut right = 0;
        for i in 0..s.len() {
            right = right.max(map[&s[i]]);
            if i == right {
                result.push((right - left + 1) as i32);
                left = i + 1;
            }
        }
        result
    }
}
// @lc code=end
