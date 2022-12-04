/*
 * @lc app=leetcode id=459 lang=rust
 *
 * [459] Repeated Substring Pattern
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn repeated_substring_pattern_v1(s: String) -> bool {
        let len = s.len();
        if len < 2 {
            return false;
        }
        for i in 1..=len / 2 {
            if len % i == 0 {
                let tmp = s.get(0..i).unwrap();
                let mut res = String::new();
                for _j in 0..len / i {
                    res += tmp;
                }
                if res == s {
                    return true;
                }
            }
        }
        false
    }

    pub fn repeated_substring_pattern(s: String) -> bool {
        (1..s.len())
            .filter(|&step| s.len() % step == 0) // 1 2
            .map(|step| { 
                (step..s.len()) // 1..4 2..4
                    .step_by(step) //1,2,3,4 2,4
                    .all(|front| &s[front..step + front] == &s[..step]) 
                    // &s[1..1+1] == &s[..1] &s[2..1+2] == &s[..2] 
                    // &s[2..2+2] == &s[..2] 
                    // Tests if [every] element of the iterator matches a predicate.
            })
            .any(|x| x) //Tests if [any] element of the iterator matches a predicate.
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
