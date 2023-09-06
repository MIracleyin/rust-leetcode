#
# @lc app=leetcode id=108 lang=python3
#
# [108] Convert Sorted Array to Binary Search Tree
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
from collections import deque
from typing import List, Optional


class Solution:
    def sortedArrayToBST_v1(self, nums: List[int]) -> Optional[TreeNode]:
        return self.traversal(nums, 0, len(nums) - 1)

    def traversal(self, nums: List[int], left, right) -> TreeNode:
        if left > right:
            return None
        
        mid = left + (right - left) // 2
        root = TreeNode(nums[mid])
        root.left = self.traversal(nums, left, mid - 1)
        root.right = self.traversal(nums, mid + 1, right)
        return root
    
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        if len(nums) == 0:
            return None
        
        root = TreeNode(0)  # 初始根节点
        nodeQue = deque()   # 放遍历的节点
        leftQue = deque()   # 保存左区间下标
        rightQue = deque()  # 保存右区间下标
        
        nodeQue.append(root)               # 根节点入队列
        leftQue.append(0)                  # 0为左区间下标初始位置
        rightQue.append(len(nums) - 1)     # len(nums) - 1为右区间下标初始位置

        while nodeQue:
            cur = nodeQue.popleft()
            left = leftQue.popleft()
            right = rightQue.popleft()

            mid = left + (right - left) // 2
            cur.val = nums[mid]

            if left <= mid - 1:
                cur.left = TreeNode(0)
                nodeQue.append(cur.left)
                leftQue.append(left)
                rightQue.append(mid - 1)
            if right >= mid + 1:
                cur.right = TreeNode(0)
                nodeQue.append(cur.right)
                leftQue.append(mid + 1)
                rightQue.append(right)
        return root


        
# @lc code=end

