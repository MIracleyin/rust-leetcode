/*
 * @lc app=leetcode id=344 lang=rust
 *
 * [344] Reverse String
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut l, mut r) = (0, s.len() - 1);

        while l < r {
            let tc = s[l];
            s[l] = s[r];
            s[r] = tc;
            
            l += 1;
            r -= 1;
        }
    }
}
// @lc code=end

