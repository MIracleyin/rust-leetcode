

/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */
struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn can_construct_v1(ransom_note: String, magazine: String) -> bool {
        let base_char = 'a';
        let mut record = vec![0; 26];

        for byte in magazine.bytes() {
            record[byte as usize - base_char as usize] += 1;
        }
        
        for byte in ransom_note.bytes() {
            record[byte as usize - base_char as usize] -= 1;
            if record[byte as usize - base_char as usize] < 0 {
                return false;
            }
        }
        true
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();

        for c in magazine.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for c in ransom_note.chars() {
            *map.entry(c).or_insert(-1) -= 1;
            if map[&c] < 0 {
                return false;
            }
        }
        true
    }
}
// @lc code=end

