#
# @lc app=leetcode id=513 lang=python3
#
# [513] Find Bottom Left Tree Value
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
    def findBottomLeftValue_v1(self, root: Optional[TreeNode]) -> int:
        self.max_depth = float('-inf')
        self.result = None
        self.traversal(root, 0)
        return self.result
    
    def traversal(self, root, depth):
        if root.left is None and root.right is None:
            if depth > self.max_depth:
                self.max_depth = depth
                self.result = root.val
        
        if root.left:
            depth += 1 # 进入下一层，加深
            self.traversal(root.left, depth)
            depth -= 1 # 返回上一层，变浅
        if root.right:
            depth += 1
            self.traversal(root.right, depth)
            depth -= 1

    def findBottomLeftValue(self, root: Optional[TreeNode]) -> int:    
        if root is None:
            return 0
        import collections
        queue = collections.deque([root])
        result = 0
        while queue:
            for i in range(len(queue)):
                cur = queue.popleft()
                if i == 0: # frist
                    result = cur.val
                if cur.left:
                    queue.append(cur.left)
                if cur.right:
                    queue.append(cur.right)
        return result
# @lc code=end

