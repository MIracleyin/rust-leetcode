use std::process::id;

/*
 * @lc app=leetcode id=134 lang=rust
 *
 * [134] Gas Station
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn can_complete_circuit_v1(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        for i in 0..cost.len() {
            let mut rest = gas[i] - cost[i];
            let mut idx = (i + 1) % cost.len();
            while rest > 0 && idx != i {
                rest += gas[idx] - cost[idx];
                idx = (idx + 1) % cost.len();
            }
            if rest >= 0 && idx == i {
                return i as i32;
            }
        }
        -1
    }
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut cur_sum = 0;
        let mut min = i32::MAX; // a cycle, min gas
        for i in 0..gas.len() {
            let mut rest = gas[i] - cost[i];
            cur_sum += rest;
            min = min.min(cur_sum); // 
        }
        
        if cur_sum < 0 {
            return -1;
        }
        if min >= 0 {
            return 0;
        }

        for i in (0..gas.len()).rev() {
            let mut rest = gas[i] - cost[i];
            min += rest;
            if min >= 0 {
                return i as i32;
            }
        }
        -1
    }
}
// @lc code=end
#[test]
fn it_work() {
    let gas = vec![1,2,3,4,5];
    let cost = vec![3,4,5,1,2];
    Solution::can_complete_circuit(gas, cost);
}