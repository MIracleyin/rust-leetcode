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

enum NodeState {
    NoCover = 0,
    Camera = 1,
    Covered = 2,
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let state = Self::traversal(&root, &mut res);
        match state {
            NodeState::NoCover => res + 1,
            _ => res,
        }
    }

    pub fn traversal(cur: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> NodeState {
        // 0 未覆盖 1 节点已设置摄像头 2 节点已覆盖
        if let Some(node) = cur {
            let node = node.borrow();

            let left_state = Self::traversal(&node.left, ans);
            let right_state = Self::traversal(&node.right, ans);
            match (left_state, right_state) {
                (NodeState::NoCover, _) | (_, NodeState::NoCover) => {
                    *ans += 1;
                    NodeState::Camera
                }
                (NodeState::Camera, _) | (_, NodeState::Camera) => NodeState::Covered,
                (_, _) => NodeState::NoCover,
            }
        } else {
            NodeState::Covered
        }
    }
}
// @lc code=end
