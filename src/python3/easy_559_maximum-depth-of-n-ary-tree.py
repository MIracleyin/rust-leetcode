#
# @lc app=leetcode id=559 lang=python3
#
# [559] Maximum Depth of N-ary Tree
#
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
# @lc code=start
"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""
import collections
class Solution:
    def maxDepth(self, root: 'Node') -> int:
        if root is None:
            return 0
        
        depth = 0
        queue = collections.deque([root])

        while queue:
            depth += 1
            for _ in range(len(queue)):
                cur = queue.popleft()
                for child in cur.children:
                    queue.append(child)

        return depth

    def maxDepth_v1(self, root: 'Node') -> int:
        if root is None:
            return 0
        
        max_depth = 1

        for child in root.children:
            max_depth = max(max_depth, self.maxDepth(child) + 1)

        return max_depth
    
    

        
# @lc code=end

