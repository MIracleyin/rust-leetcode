/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
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
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn preorder_traversal_v1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::traversal(&root, &mut res);
        res
    }

    pub fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = root {
            res.push(node.borrow().val);
            Self::traversal(&node.borrow().left, res);
            Self::traversal(&node.borrow().right, res);
        }
    }

    pub fn preorder_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = vec![root];
        while !stack.is_empty() {
            if let Some(node) = stack.pop().unwrap() {
                res.push(node.borrow().val); // pop stack to vec
                stack.push(node.borrow().right.clone());
                stack.push(node.borrow().left.clone());
            }
        }
        res
    }
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
                if node.borrow().left.is_some() {
                    stack.push(node.borrow().left.clone());
                }
                stack.push(Some(node));
                stack.push(None);
            } else {
                res.push(stack.pop().unwrap().unwrap().borrow().val)
            }
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
    }
}

