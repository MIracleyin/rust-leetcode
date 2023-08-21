#
# @lc app=leetcode id=404 lang=python3
#
# [404] Sum of Left Leaves
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
    def sumOfLeftLeaves(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        if root.left is None and root.right is None:
            return 0

        leftValue = self.sumOfLeftLeaves(root.left)
        if root.left and root.left.left is None and root.left.right is None:
            leftValue = root.left.val
        
        rightValue = self.sumOfLeftLeaves(root.right)

        sumValue = leftValue + rightValue

        return sumValue

    def sumOfLeftLeaves(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        st = [root]
        result = 0
        while st:
            cur = st.pop()
            if cur.left and cur.left.left is None and cur.left.right is None:
                result += cur.left.val
            if cur.left:
                st.append(cur.left)
            if cur.right:
                st.append(cur.right)
        
        return result
# @lc code=end

