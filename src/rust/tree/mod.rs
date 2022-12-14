mod easy_144;
mod easy_145;
mod easy_94;
mod medium_102;
mod medium_107;
mod medium_199;
mod easy_637;

mod easy_226;
// mod easy_101;
// mod easy_104;

mod medium_106;
mod medium_105;

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn get_val(&self) -> i32 {
        return self.val;
    }

    pub fn set_val(&mut self, val: i32) -> i32 {
        self.val = val;
        self.get_val()
    }

    pub fn insert(&mut self, dir: &str, node: TreeNode) {
        assert!(dir == "left" || dir == "right");
        match dir {
            "left" => {
                self.left = Some(Rc::new(RefCell::new(node)));
            }
            "right" => {
                self.right = Some(Rc::new(RefCell::new(node)));
            }
            _ => {
                panic!("Insert Error: only left and right supported");
            }
        }
    }

    pub fn build_tree_ip(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
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

        root.left = Self::build_tree_ip(inorder_left, postorder_left);
        root.right = Self::build_tree_ip(inorder_right, postorder_right);

        Some(Rc::new(RefCell::new(root)))
    }

    pub fn build_tree_pi(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
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
        root.left = Self::build_tree_pi(preorder_left, inorder_left);
        root.right = Self::build_tree_pi(preorder_right, inorder_right);
        Some(Rc::new(RefCell::new(root)))
    }

    pub fn insert_left_node(&mut self, val: i32) {
        if self.left.is_none() {
            let new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            self.left = new_node;
        }
    }

    pub fn get_left(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
        self.left.clone()
    }

    pub fn insert_right_node(&mut self, val: i32) {
        if self.right.is_none() {
            let new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            self.right = new_node;
        }
    }

    pub fn get_right(&mut self) -> Option<Rc<RefCell<TreeNode>>> {
        self.right.clone()
    }
}

trait TreeTrait {
    fn new(val: i32) -> Self;

    fn insert(self: &mut Self, val: i32) -> Result<i32, String>;

    fn search(self: &mut Self, val: i32) -> Option<Rc<RefCell<TreeNode>>>;

    fn delete(self: &mut Self, val: i32) -> Result<i32, String>;
}

#[derive(Debug)]
pub struct Tree {
    pub root: Option<Rc<RefCell<TreeNode>>>,
    pub length: usize,
}
/// reference https://www.cnblogs.com/ishenghuo/p/13582902.html
impl TreeTrait for Tree {
    fn new(val: i32) -> Self {
        let node = TreeNode::new(val);
        Tree {
            root: Some(Rc::new(RefCell::new(node))),
            length: 1,
        }
    }
    // binary search tree
    fn insert(self: &mut Self, val: i32) -> Result<i32, String> {
        todo!()
    }

    fn search(self: &mut Self, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }

    fn delete(self: &mut Self, val: i32) -> Result<i32, String> {
        todo!()
    }
}
impl Tree {
    fn get_rc(rc_rc: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref new_node_rf) = *rc_rc {
            let new_rc = Rc::clone(new_node_rf);
            Some(new_rc)
        } else {
            None
        }
    }
    fn get_val(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let rc = Tree::get_rc(node);
        return rc.unwrap().borrow().val;
    }
}

#[test]
fn test_get_val() {
    let node = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    assert_eq!(3, node.unwrap().borrow().val);
}

#[test]
fn test_tree_new() {
    let tree = Tree::new(3);
    let v1 = tree.root.unwrap().borrow().val;

    assert_eq!(3, v1);
    assert_eq!(1, tree.length);
}

#[test]
fn test_insert_tree() {
    let mut root = TreeNode::new(1);
    let mut node1 = TreeNode::new(3);
    let mut node2 = TreeNode::new(2);
    let mut node3 = TreeNode::new(5);
    let mut node4 = TreeNode::new(3);
    let mut node5 = TreeNode::new(9);
    root.insert("left", node1.clone());
    root.insert("right", node2.clone());
    node1.insert("left", node3);
    node1.insert("right", node4);
    node2.insert("right", node5);

    dbg!(root);
}
