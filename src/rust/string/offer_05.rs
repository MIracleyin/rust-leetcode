/*
 * @lc app=leetcode id=05 lang=rust
 *
 * 
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut len = s.len();
        let mut s = s.chars().collect::<Vec<char>>();

        let mut cnt = 0;
        for i in &s {
            if i.is_ascii_whitespace() {
                cnt += 1;
            }
        }

        let mut new_len = len + cnt * 2;
        s.resize(new_len, ' ');

        while len < new_len {
            len -= 1;
            new_len -= 1;
            if s[len].is_ascii_whitespace() {
                s[new_len] = '0';
                s[new_len - 1] = '2';
                s[new_len - 2] = '%';
                new_len -= 2;
            } else {
                s[new_len] = s[len];
            }
        }

        s.iter().collect::<String>()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "We are happy.".to_string();
        let res = Solution::replace_space(s);
        dbg!(res);
    }
}
