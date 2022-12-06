use core::num;

/*
 * @lc app=leetcode id=239 lang=rust
 *
 * [239] Sliding Window Maximum
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;

struct MyQueue {
    queue: VecDeque<i32>
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            queue: VecDeque::new()
        }
    }

    pub fn pop(&mut self, value: i32) {
        if !self.queue.is_empty() && *self.queue.front().unwrap() == value {
            // not empty and value still have (other will remove in push)
            self.queue.pop_front();
        }
    }

    pub fn push(&mut self, value: i32) {
        while !self.queue.is_empty() && *self.queue.back().unwrap() < value {
            self.queue.pop_back();
        }
        self.queue.push_back(value);
    }

    pub fn front(&self) -> i32 {
        *self.queue.front().unwrap()
    }
}

impl Solution {
    pub fn max_sliding_window_v1(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let win_num = nums.len() - k as usize + 1;
        if k == 1 {
            return nums;
        }
        let mut res = Vec::new();
        for i in 0..win_num {
            let mut win_vec = Vec::new();
            for j in 0..k as usize {
                win_vec.push(nums[i + j]);
            }
            let max = *win_vec.iter().max().unwrap();
            res.push(max);
        }
        res
    }

    pub fn max_sliding_window_v2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = VecDeque::with_capacity(k as usize); // save index
        let mut res = Vec::new();

        for (i, &v) in nums.iter().enumerate() {
            if i - queue.front().unwrap_or(&0) == k as usize {
                queue.pop_front();
            }
            while let Some(&idx) = queue.back() {
                if nums[idx] >= v {
                    break;
                }
                queue.pop_back();
            }
            queue.push_back(i);
            if i >= k as usize - 1 {
                res.push(nums[queue[0]]);
            }
        }
        res
    }

    pub fn max_sliding_window_v3(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = VecDeque::with_capacity(k as usize); // save index
        let mut res = Vec::new();

        for i in 0..k as usize {
            queue.push_back(nums[i]);
        }
        let max = *queue.iter().max().unwrap();
            res.push(max);

        for i in k as usize..nums.len() {
            // pop front
            queue.pop_front();
            // push back
            queue.push_back(nums[i]);
            // get max
            let max = *queue.iter().max().unwrap();
            res.push(max);
        }
        res
    }

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = MyQueue::new();
        let mut res = Vec::new();

        for i in 0..k as usize {
            queue.push(nums[i]);
        }
        res.push(queue.front());
        for i in k as usize..nums.len() {
            queue.pop(nums[i - k as usize]); // remove max element
            queue.push(nums[i]); // add the element and remove front elements that small than the element
            res.push(queue.front());
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
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let res = Solution::max_sliding_window(nums, k);
        dbg!(res);
    }
}
