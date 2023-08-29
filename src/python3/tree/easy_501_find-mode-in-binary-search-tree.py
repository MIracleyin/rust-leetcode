#
# @lc app=leetcode id=501 lang=python3
#
# [501] Find Mode in Binary Search Tree
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
from collections import defaultdict
from typing import List, Optional


class Solution:
    def traversal(self, root):
        if root is None:
            return
        self.traversal(root.left)
        self.freq_map[root.val] += 1
        self.traversal(root.right)

    def findMode_v1(self, root: Optional[TreeNode]) -> List[int]:
        self.freq_map = defaultdict(int) # defaultdict 是元素都相同的 dict
        res = []
        self.traversal(root)
        max_freq = max(self.freq_map.values())
        for k, v in self.freq_map.items():
            if v == max_freq:
                res.append(k)
        return res

    def search(self, root):
        if root is None:
            return
        
        self.search(root.left)
        if self.pre is None:
            self.count = 1
        elif self.pre.val == root.val:
            self.count += 1
        else:
            self.count = 1
        self.pre = root
        if self.count == self.max_count:
            self.res.append(self.pre.val)
        if self.count > self.max_count:
            self.max_count = self.count
            self.res = [self.pre.val]
        self.search(root.right)
        
    
    def findMode(self, root: Optional[TreeNode]) -> List[int]:
        self.count = 0
        self.max_count = 0
        self.pre = None
        self.res = []
        self.search(root)
        return self.res
        
# @lc code=end

