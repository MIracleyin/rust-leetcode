#
# @lc app=leetcode id=98 lang=python3
#
# [98] Validate Binary Search Tree
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
    def __init__(self):
        self.maxVal = float('-inf')
        self.vec = []
        self.pre = None

    def isValidBST_v1(self, root: Optional[TreeNode]) -> bool:
        self.val = []
        self.traversal(root)
        for i in range(1, len(self.val)):
            if self.val[i - 1] >= self.val[i]:
                return False
        return True


    def traversal(self, root):
        if root is None:
            return
        self.traversal(root.left)
        self.val.append(root.val)
        self.traversal(root.right)      
    
    def isValidBST_v2(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True
        left = self.isValidBST(root.left)
        if root.val > self.maxVal:
            self.maxVal = root.val
        else:
            return False
        right = self.isValidBST(root.right)
        return left and right
    
    def isValidBST_v3(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True
        left = self.isValidBST(root.left)
        if self.pre is not None and self.pre.val >= root.val:
            return False
        self.pre = root
        right = self.isValidBST(root.right)
        
        return left and right
    
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        stack = []
        cur = root
        pre = None
        while cur is not None or len(stack) > 0:
            if cur is not None:
                stack.append(cur)
                cur = cur.left
            else:
                cur = stack.pop()
                if pre is not None and pre.val >= cur.val:
                    return False
                pre = cur
                cur = cur.right

        return True

# @lc code=end

