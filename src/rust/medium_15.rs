

/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */
struct Solution;

// @lc code=start
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // v.sort_by(|a, b| b.cmp(a))
        // a + b + c = 0
        // find all (a + b) 
        // if c = -(a + b)
        let mut res = vec![];
        nums.sort(); // a
        for i in 0..nums.len() {
            if nums[i] > 0 { //i=0
                break;
            }
            if i > 0 && (nums[i] == nums[i-1]) {
                continue;
            }
            let mut set = HashSet::new();
            for j in i+1..nums.len() {
                if j > i + 2
                    && nums[j] == nums[j-1]
                    && nums[j-1] == nums[j-2] {
                        continue;
                    }
                let c = 0 - (nums[i] + nums[j]);
                if set.contains(&c) {
                    res.push(vec![nums[i], nums[j], c]);
                    set.remove(&c);
                } else {
                    set.insert(nums[j]);
                }
            }
        }
        res
    }
}
// @lc code=end

mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        let n = vec![-1,-1,2];
        let res = Solution::three_sum(n);
        dbg!(res);
        // println!("{:?}", n);
        // println!("{}", size);
    }
}

