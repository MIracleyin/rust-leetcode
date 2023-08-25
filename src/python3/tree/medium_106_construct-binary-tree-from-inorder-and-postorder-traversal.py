#
# @lc app=leetcode id=106 lang=python3
#
# [106] Construct Binary Tree from Inorder and Postorder Traversal
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
from typing import List, Optional


class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> Optional[TreeNode]:
        if not postorder:
            return None
        
        root_val = postorder[-1]
        root = TreeNode(root_val)

        separator_idx = inorder.index(root_val)
        inorder_left = inorder[:separator_idx]
        inorder_right = inorder[separator_idx + 1:]

        postorder_left = postorder[:len(inorder_left)]
        postorder_right = postorder[len(inorder_left): -1]

        root.left = self.buildTree(inorder_left, postorder_left)
        root.right = self.buildTree(inorder_right, postorder_right)

        return root
        
# @lc code=end

