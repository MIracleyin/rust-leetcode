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
                _ => {
                    if stack.is_empty() || stack.pop().unwrap() != s{
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}
// @lc code=end

