#
# @lc app=leetcode id=669 lang=python3
#
# [669] Trim a Binary Search Tree
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
    def trimBST_v1(self, root: Optional[TreeNode], low: int, high: int) -> Optional[TreeNode]:
        if root is None:
            return None
        if root.val < low:
            return self.trimBST(root.right, low, high) # 如果 root 小于，那么只有右才有符合的值
        if root.val > high:
            return self.trimBST(root.left, low, high)
        # 符合要求的
        root.left = self.trimBST(root.left, low, high)
        root.right = self.trimBST(root.right, low, high)
        return root
    
    def trimBST(self, root: Optional[TreeNode], low: int, high: int) -> Optional[TreeNode]:
        if root is None:
            return None
        
        while root and (root.val < low or root.val > high): # or here
            if root.val < low:
                root = root.right
            if root.val > high:
                root = root.left
        
        cur = root
        while cur:
            while cur.left and cur.left.val < low:
                cur.left = cur.left.right
            cur = cur.left

        cur = root
        while cur:
            while cur.right and cur.right.val > high:
                cur.right = cur.right.left
            cur = cur.right
        
        return root


# @lc code=end

