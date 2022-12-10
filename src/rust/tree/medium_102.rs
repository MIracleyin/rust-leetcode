/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
 */
use crate::rust::tree::TreeNode;
struct Solution;

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
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut que = VecDeque::new();
        if root.is_some() {
            que.push_back(root);
        }
        while !que.is_empty() {
            let len = que.len();
            let mut subres = vec![];
            for _ in 0..len {
                let node = que.pop_front().unwrap().unwrap();
                subres.push(node.borrow().val);
                if node.borrow().left.is_some() {
                    que.push_back(node.borrow().left.clone())
                }
                if node.borrow().right.is_some() {
                    que.push_back(node.borrow().right.clone())
                }
            }
            res.push(subres);
        }
        res
    }
}
// @lc code=end
