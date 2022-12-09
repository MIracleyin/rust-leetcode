/*
 * @lc app=leetcode id=105 lang=rust
 *
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 */
struct Solution;
use crate::rust::tree::TreeNode;
use std::process::id;
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = preorder.len();
        if len == 0 {
            return None;
        }

        let val = preorder[0];

        // get root val idx
        let idx = inorder.iter().position(|&x| x == val).unwrap();

        // cur inorder array
        let inorder_left = inorder[..idx].to_vec();
        let inorder_right = inorder[idx + 1..].to_vec();

        let preorder_left = preorder[1..idx + 1].to_vec();
        let preorder_right = preorder[idx + 1..].to_vec();
        let mut root = TreeNode::new(val);
        root.left = Self::build_tree(preorder_left, inorder_left);
        root.right = Self::build_tree(preorder_right, inorder_right);

        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end
