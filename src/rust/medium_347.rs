
/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */
struct Solution;
// @lc code=start
use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};
impl Solution {
    pub fn top_k_frequent_v1(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        nums.into_iter().for_each(|k| {
            *map.entry(k).or_insert(0) += 1;
        });
        let mut heap = BinaryHeap::with_capacity(k as usize);

        for (k, v) in map {
            if heap.len() == heap.capacity() {
                if *heap.peek().unwrap() < (Reverse(v), k) {
                    continue;
                } else {
                    heap.pop();
                }

            }
            heap.push((Reverse(v), k))
        }
        heap.into_iter().map(|(_, k)| k).collect()
    }

    pub fn top_k_frequent_v2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        nums.into_iter().for_each(|k| {
            *map.entry(k).or_insert(0) += 1;
        });
        let mut mv = map.iter().collect::<Vec<_>>();
        mv.sort_by(|a, b| b.1.cmp(&a.1));
        mv[0..k as usize].into_iter().map(|e| *e.0).collect()
    }

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        nums.into_iter().for_each(|k| {
            *map.entry(k).or_insert(0) += 1;
        });
        let mut heap = BinaryHeap::new();

        for (num, cnt) in map.iter() {
            heap.push((cnt, num));
        }

        let mut res = Vec::new();
        for _ in 0..k {
            res.push(heap.pop().unwrap().1.to_owned())
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1,1,1,2,2,3];
        let k = 2;
        let res = Solution::top_k_frequent(nums, k);
        dbg!(res);
    }
}