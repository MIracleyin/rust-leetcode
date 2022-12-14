/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        const MAP: [&str; 10] = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let mut result: Vec<String> = Vec::new();
        let mut s: String = String::new();
        Self::backtracking(&mut result, &mut s, &MAP, &digits, 0);
        result
    }

    pub fn backtracking(
        result: &mut Vec<String>,
        s: &mut String,
        map: &[&str; 10],
        digits: &String,
        index: usize,
    ) {
        let len = digits.len();
        if len == index {
            result.push(s.to_string());
            return;
        }

        let digit = digits.chars().nth(index).unwrap().to_digit(10).unwrap() as usize;
        let letters = map[digit];
        for i in letters.chars() {
            s.push(i);
            Self::backtracking(result, s, &map, &digits, index + 1);
            s.pop();
        }
    }
}
// @lc code=end
#[test]
fn it_works() {
    let digits = "23".to_string();
    let res = Solution::letter_combinations(digits);
    dbg!(res);
}
