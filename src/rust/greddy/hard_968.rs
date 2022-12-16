/*
 * @lc app=leetcode id=968 lang=rust
 *
 * [968] Binary Tree Cameras
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
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        if Self::traversal(&root, &mut res) == 0 {
            res += 1;
        }
        res
    }

    pub fn traversal(cur: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        // 0 未覆盖 1 节点已设置摄像头 2 节点已覆盖
        if let Some(node) = cur {
            let node = node.borrow();

            let left = Self::traversal(&node.left, ans);
            let right = Self::traversal(&node.right, ans);

            // 左右节点都被覆盖
            if left == 2 && right == 2 {
                return 0; // 无覆盖
            }

            // left == 0 right == 0 左右无覆盖
            // left == 0 right == 1 左节点无覆盖 右节点有摄像头
            // left == 1 right == 0 左节点有摄像头 左节点无覆盖
            // left == 0 right == 2 左节点无覆盖 右节点有覆盖
            // left == 2 right == 0 左节点有覆盖 右节点无覆盖
            if left == 0 || right == 0 {
                *ans += 1;
                return 1;
            }

            // left == 1 right == 1 左节点有摄像头 右节点有摄像头
            // left == 1 right == 2 左节点有摄像头 右节点覆盖
            // left == 2 right == 1 左节点覆盖 右节点有摄像头
            if left == 1 || right == 1 {
                return 2; // 已覆盖
            }
        } else {
            return 2;
        }
        -1
    }
}
// @lc code=end
