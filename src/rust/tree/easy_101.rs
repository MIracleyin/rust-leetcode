/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::recur(
            &root.as_ref().unwrap().borrow().left,
            &root.as_ref().unwrap().borrow().right,
        )
    }

    // 确定递归函数
    pub fn recur(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // 确定终止条件
        // left null right not null
        // left not null right null
        // left null right null
        // left not null right not null (val)
        match (left, right) {
            (None, None) => true,
            (Some(n1), Some(n2)) => {
                return n1.borrow().val == n2.borrow().val
                    && Self::recur(n1.borrow().left, &n2.borrow().right)
                    && Self::recur(&n1.borrow().right, &n2.borrow().left)
            }
            _ => false,
        }
    }
}
// @lc code=end
