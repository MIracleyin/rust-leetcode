#
# @lc app=leetcode id=112 lang=python3
#
# [112] Path Sum
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
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if root is None:
            return False
        return self.traversal(root, targetSum - root.val)

    def traversal(self, cur: TreeNode, count: int) -> bool:
        if cur.left is None and cur.right is None and count == 0:
            return True
        elif cur.left is None and cur.right is None:
            return False
        
        if cur.left:
            count -= cur.left.val
            if self.traversal(cur.left, count):
                return True
            count += cur.left.val
        
        if cur.right:
            count -= cur.right.val
            if self.traversal(cur.right, count):
                return True
            count += cur.right.val

        return False
    
    def hasPathSum_v1(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if root is None:
            return False
        
        st = [(root, root.val)]
        while st:
            cur, path_sum = st.pop()
            if cur.left is None and cur.right is None and path_sum == targetSum:
                return True
            if cur.left:
                st.append((cur.left, path_sum + cur.left.val))
            if cur.right:
                st.append((cur.right, path_sum + cur.right.val))
        return False



  
# @lc code=end

