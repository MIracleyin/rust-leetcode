#
# @lc app=leetcode id=110 lang=python3
#
# [110] Balanced Binary Tree
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
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if self.getHight(root) != -1:
            return True
        else:
            return False

    def getHight(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        if (left_hight := self.getHight(root.left)) == -1:
            return -1
        if (right_hight := self.getHight(root.right)) == -1:
            return -1
        if abs(left_hight - right_hight) > 1:
            return -1
        else:
            return 1 + max(left_hight, right_hight)


        
# @lc code=end

