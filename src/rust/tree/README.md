# 二叉树

## 二叉树的种类
结构形式
- 满二叉树：如果一棵二叉树只有度为0的结点和度为2的结点，并且度为0的结点在同一层上，则这棵二叉树为满二叉树。
- 完全二叉树：除了最底层节点可能没填满外，其余每层节点数都达到最大值，并且最下面一层的节点都集中在该层最左边的若干位置。

数值
- 二叉搜索树：二叉搜索树是一个有序树
    - 若它的左子树不空，则左子树上所有结点的值均小于它的根结点的值；
    - 若它的右子树不空，则右子树上所有结点的值均大于它的根结点的值；
    - 它的左、右子树也分别为二叉排序树
- 平衡二叉搜索树 AVL（Adelson-Velsky and Landis）：它是一棵空树或它的左右两个子树的高度差的绝对值不超过1，并且左右两个子树都是一棵平衡二叉树。

## 二叉树的存储
链式（指针）
顺序（数组）

## 二叉树的遍历
深度优先
前中后指中间节点的位置
- 前序遍历（中左右）
- 中序遍历（左中右）
- 后序遍历（左右中）
广度优先
- 层序遍历

## 二叉树的定义

```Rust
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
```
