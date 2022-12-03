/*
 * @lc app=leetcode id=151 lang=rust
 *
 * [151] Reverse Words in a String
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        // remove whilespace
        Self::remove_extra_spaces(&mut s);

        // reverse string
        let len = s.len();
        Self::reverse(&mut s, 0, len - 1);

        // reverse word
        let mut start = 0;

        for i in 0..=len {
            if i == len || s[i].is_ascii_whitespace()  { // if s[i] is ' ' or end of s
                Self::reverse(&mut s, start, i - 1);
                start = i + 1;
            }
        }
        s.iter().collect::<String>()

    }

    pub fn remove_extra_spaces(s: &mut Vec<char>) {
        let mut slow = 0;
        let len = s.len();
        let mut pos = 0;
        while pos < len {
            if !s[pos].is_ascii_whitespace() { // if this pos is not whiltspace
                if slow != 0 {
                    s[slow] = ' ';
                    slow += 1;
                }

                while pos < len && !s[pos].is_ascii_whitespace() { // until s[pos] not a whitespace
                    s[slow] = s[pos];
                    slow += 1;
                    pos += 1;
                }
            }
            pos += 1;
        }
        s.resize(slow, ' ');
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = "  he wo".to_string();
        // Solution::reverse(&mut s, 0, len - 1);
        let res = Solution::reverse_words(s.clone());
        dbg!(res);
    }
}