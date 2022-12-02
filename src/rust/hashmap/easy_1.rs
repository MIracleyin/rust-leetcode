
/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            if let Some(k) = map.get(&(target - nums[i])) {
                if *k != i {
                    return vec![*k as i32, i as i32];
                }
            }
            map.insert(nums[i], i);
        }
        panic!("not found")
    }
}
// @lc code=end
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        let nums = vec![2,7,11,15];
        let target = 9;
        println!("{:#?}", Solution::two_sum(nums, target));
    }
}


