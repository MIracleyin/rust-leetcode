/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 * [242] 有效的字母异位词
 */


struct Solution;

// @lc code=start

use std::collections::HashMap;
impl Solution {
    pub fn is_anagram_v1(s: String, t: String) -> bool {
        // s -> char map
        // t -> char map
        // if all char count is same
        // true
        let mut record = vec![0;26];

        let base = 'a';

        for c in s.chars() {
            record[c as usize - base as usize] += 1;
        }

        for c in t.chars() {
            record[c as usize - base as usize] -= 1;
        }

        record.iter().filter(|x| **x != 0).count() == 0
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        // s -> char map
        // t -> char map
        // if all char count is same
        // true
        let mut recore= HashMap::new();

        for c in s.chars() {
            let cnt = recore.entry(c).or_insert(0);
            *cnt += 1;
        }

        for c in t.chars() {
            let cnt = recore.entry(c).or_insert(0);
            *cnt -= 1;
        }

        for (_k, v) in recore.iter() {
            if *v != 0 {
                return false
            }
        }
        true
    }
}
// @lc code=end

