use std::path;

/*
 * @lc app=leetcode id=131 lang=rust
 *
 * [131] Palindrome Partitioning
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut path: Vec<String> = Vec::new();
        let sub_str: Vec<char> = s.chars().collect();

        Self::backtracking(&mut result, &mut path, &sub_str, 0);
        result
    }

    pub fn backtracking(
        result: &mut Vec<Vec<String>>,
        path: &mut Vec<String>,
        sub_str: &Vec<char>,
        start_idx: usize,
    ) {
        if start_idx >= sub_str.len() {
            result.push(path.clone());
            return;
        }

        for i in start_idx..sub_str.len() {
            if Self::is_palindrome(sub_str, start_idx, i) {
                let s: String = sub_str[start_idx..i + 1].into_iter().collect();
                path.push(s);
                Self::backtracking(result, path, sub_str, i + 1);
                path.pop();
            }
        }
    }

    pub fn is_palindrome(s: &Vec<char>, start: usize, end: usize) -> bool {
        let (mut start, mut end) = (start, end);
        while start < end {
            if s[start] != s[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}
// @lc code=end
