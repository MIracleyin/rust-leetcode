/*
 * @lc app=leetcode id=135 lang=rust
 *
 * [135] Candy
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy_vec = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candy_vec[i] = candy_vec[i - 1] + 1;
            }
        }
        for i in (0..(ratings.len() - 1)).rev() {
            if ratings[i] > ratings[i + 1] {
                candy_vec[i] = candy_vec[i].max(candy_vec[i + 1] + 1);
            }
        }
        candy_vec.iter().sum()
    }
}
// @lc code=end
