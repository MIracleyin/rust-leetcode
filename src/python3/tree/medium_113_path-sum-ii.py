#
# @lc app=leetcode id=113 lang=python3
#
# [113] Path Sum II
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
    def __init__(self):
        self.result = []
        self.path = []

    def pathSum_v1(self, root: Optional[TreeNode], targetSum: int) -> List[List[int]]:
        self.result.clear()
        self.path.clear()
        if root is None:
            return self.result
        self.path.append(root.val)
        self.traversal(root, targetSum - root.val)
        return self.result

    def traversal(self, cur, count): 
        if cur.left is None and cur.right is None and count == 0:
            self.result.append(self.path[:])
            return
        if cur.left is None and cur.right is None:
            return
        
        if cur.left:
            self.path.append(cur.left.val)
            count -= cur.left.val
            self.traversal(cur.left, count)
            count += cur.left.val
            self.path.pop()
            
            
        if cur.right:
            self.path.append(cur.right.val)
            count -= cur.right.val
            self.traversal(cur.right, count)
            count += cur.right.val
            self.path.pop()
            
        return False
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> List[List[int]]:
        if root is None:
            return []
        st = [(root, [root.val])]
        res = []
        while st:
            cur, path = st.pop()
            if cur.left is None and cur.right is None and sum(path) == targetSum:
                res.append(path)
            if cur.left:
                st.append((cur.left, path + [cur.left.val]))
            if cur.right:
                st.append((cur.right, path + [cur.right.val]))
            
        return res
# @lc code=end

