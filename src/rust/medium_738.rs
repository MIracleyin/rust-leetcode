/*
 * @lc app=leetcode id=738 lang=rust
 *
 * [738] Monotone Increasing Digits
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn monotone_increasing_digits_v1(n: i32) -> i32 {
        for i in (0..=n).rev() {
            if Self::is_vaild(i) {
                return i;
            }
        }
        0
    }

    pub fn is_vaild(mut num: i32) -> bool {
        let mut max = 10;
        while num != 0 {
            let t = num % 10;
            if max >= t {
                max = t;
            } else {
                return false;
            }
            num /= 10;
        }
        true
    }

    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut str_num = n
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        let mut flag = str_num.len();

        for i in (1..str_num.len()).rev() {
            if str_num[i - 1] > str_num[i] {
                flag = i;
                str_num[i - 1] -= 1;
            }
        }
        for i in flag..str_num.len() {
            str_num[i] = 9;
        }
        str_num.iter().fold(0, |acc, x| acc * 10 + x)
    }
}
// @lc code=end
