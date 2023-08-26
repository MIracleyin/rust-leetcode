#
# @lc app=leetcode id=654 lang=python3
#
# [654] Maximum Binary Tree
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
    def constructMaximumBinaryTree(self, nums: List[int]) -> Optional[TreeNode]:
        if len(nums) == 1:
            return TreeNode(nums[0])
        
        max_value = 0
        max_value_idx = 0
        for i in range(len(nums)):
            if nums[i] > max_value:
                max_value = nums[i]
                max_value_idx = i
        
        root = TreeNode(max_value)

        if max_value_idx > 0: # 如果左list还存在
            left_nums = nums[:max_value_idx]
            root.left = self.constructMaximumBinaryTree(left_nums)
        if max_value_idx < len(nums) - 1: # 如果右list还存在
            right_nums = nums[max_value_idx+1:]
            root.right = self.constructMaximumBinaryTree(right_nums)
        return root
    
    def constructMaximumBinaryTree_v1(self, nums: List[int]) -> Optional[TreeNode]:
        return self.traversal(nums, 0, len(nums))

    def traversal(self, nums: List[int], left: int, right: int) -> TreeNode:
        if left >= right:
            return None
        max_value_idx = left
        for i in range(left + 1, right):
            if nums[i] >= nums[max_value_idx]:
                max_value_idx = i
        root = TreeNode(nums[max_value_idx])
        root.left = self.traversal(nums, left, max_value_idx)
        root.right = self.traversal(nums, max_value_idx+1, right)
        return root
# @lc code=end

