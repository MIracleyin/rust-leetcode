/*
 * @lc app=leetcode id=459 lang=rust
 *
 * [459] Repeated Substring Pattern
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();
        if len < 2 {
            return false;
        }
        for i in 1..=len/2 {
            if len % i == 0 {
                let tmp = s.get(0..i).unwrap();
                let mut res = String::new();
                for _j in 0..len/i {
                    res += tmp;
                }
                if res == s {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "abab".to_string();
        let res = Solution::repeated_substring_pattern(s);
        dbg!(res);
    }
}

