#
# @lc app=leetcode id=101 lang=python3
#
# [101] Symmetric Tree
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
    def isSymmetric_v1(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True
        return self.compare(root.left, root.right)
        
    def compare(self, left, right) -> bool:
        # 排除空节点
        if left is None and right is not None: # 左右不对称 
            return False
        elif left is not None and right is None: # 左右不对称
            return False
        elif left is None and right is None: # 左右都空
            return True
        elif left.val != right.val: # 左右不对称，值不对
            return False
        
        outside = self.compare(left.left, right.right)
        inside = self.compare(left.right, right.left)
        is_same = outside and inside
        return is_same
    
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True
        
        st = []
        st.append(root.left)
        st.append(root.right)
        while st:
            right_node = st.pop()
            left_node = st.pop()
            if left_node is None and right_node is None:
                continue
            if left_node is None or right_node is None or left_node.val != right_node.val:
                return False
            st.append(left_node.left)
            st.append(right_node.right)
            st.append(left_node.right)
            st.append(right_node.left)
        return True

# @lc code=end

