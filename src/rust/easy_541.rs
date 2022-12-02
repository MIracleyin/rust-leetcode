/*
 * @lc app=leetcode id=541 lang=rust
 *
 * [541] Reverse String II
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k  = k as usize;
        let len = s.len();
        let mut s = s.chars().collect::<Vec<_>>();
        for i in (0..len).step_by(2 * k) {
            if i + k < s.len() {
                Self::reverse(&mut s, i, i + k - 1);
            } else {
                Self::reverse(&mut s, i, len - 1);
            }
        }
        s.iter().collect::<String>()
    }

    pub fn reverse(s: &mut Vec<char>, mut l: usize, mut r: usize) {
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

