/*
 * @lc app=leetcode id=93 lang=rust
 *
 * [93] Restore IP Addresses
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let len = s.len();
        if len < 4 || len > 12 {
            return result;
        }
        let mut s: Vec<char> = s.chars().collect();
        Self::backtracking(&mut result, &mut s, 0, 0);
        result
    }

    pub fn backtracking(result: &mut Vec<String>, s: &mut Vec<char>, start_idx: usize, mut point_num: i32) {
        let len = s.len();
        if point_num == 3 {
            if Self::is_valid(s, start_idx, len - 1) {
                result.push(s.iter().collect::<String>());
            }
            return;
        }
        for i in start_idx..len {
            if Self::is_valid(s, start_idx, i) {
                point_num += 1;
                s.insert(i + 1, '.');
                Self::backtracking(result, s, i + 2, point_num);
                point_num -= 1;
                s.remove(i + 1);
            } else {
                break;
            }
        }

    }

    pub fn is_valid(s: &Vec<char>, start: usize, end: usize) -> bool {
        if start > end {
            return false;
        }
        if s[start] == '0' && start != end {
            return false;
        }
        let mut num = 0;
        for i in start..=end {
            if s[i] >'9' || s[i] < '0' {
                return false;
            }
            if let Some(digit) = s[i].to_digit(10) {
                num = num * 10 + digit;
            }
            if num > 255 {
                return false;
            }
        }
        true
    }
}
// @lc code=end

