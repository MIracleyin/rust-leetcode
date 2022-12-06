/*
 * @lc app=leetcode id=1047 lang=rust
 *
 * [1047] Remove All Adjacent Duplicates In String
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut stack = Vec::new();

        while let Some(c) = s.pop() {
            if stack.is_empty() || *stack.last().unwrap() != c {
                stack.push(c);
            } else {
                stack.pop();
            }
        }

        stack.into_iter().rev().collect::<String>()
    }
}
// @lc code=end

