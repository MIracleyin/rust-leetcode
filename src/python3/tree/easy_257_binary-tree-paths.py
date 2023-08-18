#
# @lc app=leetcode id=257 lang=python3
#
# [257] Binary Tree Paths
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
from ast import List
from typing import Optional


class Solution:
    def binaryTreePaths_v1(self, root: Optional[TreeNode]) -> List:
        if root is None:
            return []
        path, result = [], []
        self.traversal(root, path, result)
        return result

    def traversal(self, cur, path, result):
        path.append(cur.val)
        if cur.left is None and cur.right is None:
            one_path = "->".join(map(str, path))
            result.append(one_path)
            return
        
        if cur.left:
            self.traversal(cur.left, path, result)
            path.pop()
        if cur.right:
            self.traversal(cur.right, path, result)
            path.pop()
    
    def binaryTreePaths(self, root: Optional[TreeNode]) -> List:
        stack, path_st, result = [root], [str(root.val)], []

        while stack:
            cur = stack.pop()
            path = path_st.pop()
            if cur.left is None and cur.right is None:
                result.append(path)
            if cur.left:
                stack.append(cur.left)
                path_st.append(path + f"->{cur.left.val}")
            if cur.right:
                stack.append(cur.right)
                path_st.append(path + f"->{cur.right.val}")
        return result
# @lc code=end

