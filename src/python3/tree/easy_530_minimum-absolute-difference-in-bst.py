# @before-stub-for-debug-begin
from python3problem530 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=530 lang=python3
#
# [530] Minimum Absolute Difference in BST
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
    def getMinimumDifference(self, root: Optional[TreeNode]) -> int:
        self.vec = []
        self.traversal(root)
        if len(self.vec) < 2:
            return 0
        res = float('inf')
        for i in range(1, len(self.vec)):
            res = min(res, self.vec[i] - self.vec[i-1])
            # if (self.vec[i] - self.vec[i - 1]) < res:
            #     res = (self.vec[i] - self.vec[i-1])
        return res
            
        
    def traversal(self, root):
        if root is None:
            return
        self.traversal(root.left)
        self.vec.append(root.val)
        self.traversal(root.right)

# @lc code=end

