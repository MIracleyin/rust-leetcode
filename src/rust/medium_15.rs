/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */
struct Solution;

// @lc code=start
use std::{collections::{HashMap, HashSet}, cmp::Ordering};
impl Solution {
    pub fn three_sum_v1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // v.sort_by(|a, b| b.cmp(a))
        // a + b + c = 0
        // find all (a + b)
        // if c = -(a + b)
        let mut res = vec![];
        nums.sort(); // a
        for i in 0..nums.len() {
            if nums[i] > 0 {
                //i=0
                break;
            }
            if i > 0 && (nums[i] == nums[i - 1]) {
                // ？
                continue;
            }
            let mut set = HashSet::new();
            for j in i + 1..nums.len() {
                if j > i + 2
                    && nums[j] == nums[j-1] // ？
                    && nums[j-1] == nums[j-2]
                {
                    // ？
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

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }

            if i > 0 && (nums[i] == nums[i - 1]) {
                continue;
            }
            // a + b + c
            let (mut l, mut r) = (i + 1, nums.len() - 1);

            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[l], nums[r]]);
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        r -= 1;
                        l += 1;
                    },
                    Ordering::Greater => {
                        r -= 1;
                    },
                    Ordering::Less => {
                        l += 1;
                    }
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
        let n = vec![-1, -1, 2];
        let res = Solution::three_sum(n);
        dbg!(res);
        // println!("{:?}", n);
        // println!("{}", size);
    }
}
