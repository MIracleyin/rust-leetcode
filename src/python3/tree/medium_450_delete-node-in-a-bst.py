#
# @lc app=leetcode id=450 lang=python3
#
# [450] Delete Node in a BST
#
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from typing import Optional
class Solution:
    def deleteNode_v1(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:
        if root is None:
            return None
        
        if root.val == key:
            if root.left is None and root.right is None:
                return None # remove
            elif root.left is not None and root.right is None:
                return root.left
            elif root.left is None and root.right is not None:
                return root.right
            else: # both not None
                cur = root.right # rigth as new root
                while cur.left is not None:
                    cur = cur.left # until right's left leaf node
                cur.left = root.left
                return root.right
        if root.val > key:
            root.left = self.deleteNode(root.left, key)
        if root.val < key:
            root.right = self.deleteNode(root.right, key)
        
        return root
    
    def deleteNode(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:
        pass
# @lc code=end

