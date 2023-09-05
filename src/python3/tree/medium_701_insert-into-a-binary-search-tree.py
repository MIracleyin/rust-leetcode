# @before-stub-for-debug-begin
from python3problem701 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=701 lang=python3
#
# [701] Insert into a Binary Search Tree
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
    def insertIntoBST_v1(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        if root is None:
            return TreeNode(val)
        
        if root.val < val: # right
            root.right = self.insertIntoBST(root.right, val)
        else:
            root.left = self.insertIntoBST(root.left, val)
        return root
    def insertIntoBST(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        if root is None:
            return TreeNode(val)
        
        cur = root
        pre = root
        while cur:
            pre = cur
            if cur.val > val: #left
                cur = cur.left
            else:
                cur = cur.right
            
        node = TreeNode(val)
        if pre.val > val:
            pre.left = node
        else:
            pre.right = node
        return root


        
# @lc code=end

