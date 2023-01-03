/*
 * @lc app=leetcode id=474 lang=rust
 *
 * [474] Ones and Zeroes
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        // dp[i][j]：最多有i个0和j个1的strs的最大子集的大小为dp[i][j]
        let (m, n) = (m as usize, n as usize);
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        for str in strs {
            let chars = str.chars().collect::<Vec<char>>();
            let (mut one_num, mut zero_num) = (0, 0);
            for c in chars.iter() {
                if c == &'0' {
                    zero_num += 1; // all zero 
                } else {
                    one_num += 1; // all one
                }
            }
            for i in (zero_num..=m).rev() {
                for j in (one_num..=n).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - zero_num][j - one_num] + 1);
                }
            }
        }
        dp[m][n]
    }
}
// @lc code=end
#[test]
fn it_works() {
    let strs = vec!["10".to_string(), "0001".to_string(), "111001".to_string(), "1".to_string(), "0".to_string()];
    let (m, n) = (3, 3);
    Solution::find_max_form(strs, m, n);
}
