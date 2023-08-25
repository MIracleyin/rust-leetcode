# @before-stub-for-debug-begin
from python3problem105 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=105 lang=python3
#
# [105] Construct Binary Tree from Preorder and Inorder Traversal
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
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        # if preorder is None:
        #     return None
        if not preorder: # []
            return None

        root_val = preorder[0]
        root = TreeNode(root_val)

        separator_idx = inorder.index(root_val) # 1
        inorder_left = inorder[:separator_idx] # [) 9
        inorder_right = inorder[separator_idx + 1:] # [1+1:] 15,20,7

        preorder_left = preorder[1: 1 + len(inorder_left)] # 9
        preorder_right = preorder[1 + len(inorder_left):]

        root.left = self.buildTree(preorder_left, inorder_left)
        root.right = self.buildTree(preorder_right, inorder_right)

        return root


# @lc code=end

