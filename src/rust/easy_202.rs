
/*
 * @lc app=leetcode id=202 lang=rust
 *
 * [202] Happy Number
 */
struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn get_sum(mut n:i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += (n % 10) * (n % 10);
            n /= 10;
        }
        sum
    }

    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut record = HashSet::new();
        loop {
            let sum = Self::get_sum(n);
            if sum == 1 {
                return true;
            }
            if record.contains(&sum) { // 
                return false;
            } else {
                record.insert(sum);
            }
            n = sum;
        }
    }
}
// @lc code=end

