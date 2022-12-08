/*
 * @lc app=leetcode id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
 */
use crate::rust::tree::TreeNode;
pub struct Solution;
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal_v1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::traversal(&root, &mut res);
        res
    }
    pub fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::traversal(&node.borrow().left, res);
            res.push(node.borrow().val);
            Self::traversal(&node.borrow().right, res);
        }
    }
    pub fn inorder_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = Vec::new();
        let mut cur = root;
        while !stack.is_empty() || cur.is_some() {
            while let Some(node) = cur {
                cur = node.borrow().left.clone();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                res.push(node.borrow().val); // mid
                cur = node.borrow().right.clone();
            }
        }
        res
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = Vec::new();
        if root.is_some() {
            stack.push(root);
        }
        while !stack.is_empty() {
            if let Some(node) = stack.pop().unwrap() {
                if node.borrow().right.is_some() {
                    stack.push(node.borrow().right.clone());
                }
                stack.push(Some(node.clone()));
                stack.push(None);
                if node.borrow().left.is_some() {
                    stack.push(node.borrow().left.clone());
                }
            } else {
                res.push(stack.pop().unwrap().unwrap().borrow().val)
            }
        }
        res
    }
}
// @lc code=end
