use std::{cell::RefCell, rc::Rc};

mod array;
mod hashmap;
mod listnode;
mod stack_queue;
mod string;
mod easy_144;
mod easy_145;
mod easy_94;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}


