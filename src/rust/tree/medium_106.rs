/*
 * @lc app=leetcode id=106 lang=rust
 *
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = postorder.len();
        if len == 0 {
            return None;
        }

        // get root val
        let val = postorder[len - 1];

        // get root val idx
        let idx = inorder.iter().position(|&x| x == val).unwrap();

        // cut inorder array
        let inorder_left = inorder[..idx].to_vec();
        let inorder_right = inorder[idx + 1..].to_vec();

        let postorder_left = postorder[..idx].to_vec();
        let postorder_right = postorder[idx..len - 1].to_vec();
        let mut root = TreeNode::new(val);

        root.left = Self::build_tree(inorder_left, postorder_left);
        root.right = Self::build_tree(inorder_right, postorder_right);

        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn it_works() {
        let inorder = vec![9,3,15,20,7];
        let postorder = vec![9,15,7,20,3];
        let res = Solution::build_tree(inorder, postorder);
        dbg!(res);
    }
}
