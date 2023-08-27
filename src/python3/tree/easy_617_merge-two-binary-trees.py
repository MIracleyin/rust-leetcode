# @before-stub-for-debug-begin
from python3problem617 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=617 lang=python3
#
# [617] Merge Two Binary Trees
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
from collections import deque

class Solution:
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        if root1 is None:
            return root2
        if root2 is None:
            return root1
        
        root1.val += root2.val
        root1.left = self.mergeTrees(root1.left, root2.left)
        root1.right = self.mergeTrees(root1.right, root2.right)
        return root1
    
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        if root1 is None:
            return root2
        if root2 is None:
            return root1

        queue = deque()
        queue.append(root1)
        queue.append(root2)

        while queue:
            cur1 = queue.popleft()
            cur2 = queue.popleft()

            if cur1.left and cur2.left:
                queue.append(cur1.left)
                queue.append(cur2.left)
            if cur1.right and cur2.right:
                queue.append(cur1.right)
                queue.append(cur2.right)
            
            cur1.val += cur2.val
            if cur1.left is None and cur2.left:
                cur1.left = cur2.left
            if cur1.right is None and cur2.right:
                cur1.right = cur2.right
            
        return root1

# @lc code=end

