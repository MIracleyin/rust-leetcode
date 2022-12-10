/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 */
use crate::rust::tree::TreeNode;
pub struct Solution;
use std::borrow::Borrow;
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
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        std::cmp::max(
            Self::max_depth(root.clone().unwrap().borrow().left.clone()),
            Self::max_depth(root.unwrap().borrow().right.clone()),
        ) + 1
    }
}
// @lc code=end

