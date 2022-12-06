/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let mut s = s.chars().collect::<Vec<char>>();
        let len = s.len();
        if len % 2 != 0 {
            return false;
        }
        while let Some(s) = s.pop() {
            match s {
                ')' => stack.push('('),
                ']' => stack.push('['),
                '}' => stack.push('{'),
                _ => { // left parentheses
                    if stack.is_empty() || stack.pop().unwrap() != s{
                        return false;
                    } // if stack is empty (no left parentheses match)
                      // if stack last element 
                }
            }
        }
        stack.is_empty()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "{[((]}))".to_string();
        let res = Solution::is_valid(s);
        assert_eq!(res, true);
    }
}