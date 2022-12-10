/*
 * @lc app=leetcode id=199 lang=rust
 *
 * [199] Binary Tree Right Side View
 */
struct Solution;
use crate::rust::tree::TreeNode;
// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut que = VecDeque::new();
        if root.is_some() {
            que.push_back(root);
        }
        while !que.is_empty() {
            let len = que.len();
            for i in 0..len {
                let node = que.pop_front().unwrap().unwrap();
                if i == len - 1 {
                    res.push(node.borrow().val);
                }
                if node.borrow().left.is_some() {
                        que.push_back(node.borrow().left.clone());
                    }
                if node.borrow().right.is_some() {
                        que.push_back(node.borrow().right.clone());
                    }
            }
        }
        res
    }
}
// @lc code=end

