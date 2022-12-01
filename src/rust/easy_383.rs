/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
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
}
// @lc code=end

