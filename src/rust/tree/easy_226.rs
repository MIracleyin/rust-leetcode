/*
 * @lc app=leetcode id=226 lang=rust
 *
 * [226] Invert Binary Tree
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
impl Solution {
    pub fn invert_tree_v1(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
            node.borrow_mut().left = Self::invert_tree_v1(right);
            node.borrow_mut().right = Self::invert_tree_v1(left);
        }
        root
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        if root.is_some() {
            stack.push(root.clone());
        }
        while !stack.is_empty() {
            if let Some(node) = stack.pop().unwrap() {
                let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
                stack.push(right.clone());
                stack.push(left.clone());
                node.borrow_mut().left = right;
                node.borrow_mut().right = left;
            }
        }
        root
    }
}
// @lc code=end

