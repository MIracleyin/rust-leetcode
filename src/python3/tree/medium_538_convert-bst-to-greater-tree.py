#
# @lc app=leetcode id=538 lang=python3
#
# [538] Convert BST to Greater Tree
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
    def convertBST_v1(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        self.pre = 0
        self.traversal(root)
        return root

    def traversal(self, cur):
        if cur is None:
            return
        self.traversal(cur.left)
        cur.val += self.pre
        self.pre = cur.val
        self.traversal(cur.right)

    def convertBST(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root is None:
            return
        
        stack = []
        cur = root
        pre = 0
        while cur or stack:
            if cur:
                stack.append(cur)
                cur = cur.right
            else:
                cur = stack.pop()
                cur.val += pre
                pre = cur.val
                cur = cur.left
        return root

# @lc code=end

