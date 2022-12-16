/*
 * @lc app=leetcode id=860 lang=rust
 *
 * [860] Lemonade Change
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;
        // let mut twenty = 0;
        for bill in bills {
            if bill == 5 {
                five += 1;
            }
            if bill == 10 {
                if five <= 0 {
                    return false;
                } 
                ten += 1;
                five -= 1;
            }
            if bill == 20 {
                if five > 0 && ten > 0 {
                    five -= 1;
                    ten -= 1;
                    // twenty += 1;
                } else if five >= 3 {
                    five -= 3;
                    // twenty += 1;
                } else {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end

