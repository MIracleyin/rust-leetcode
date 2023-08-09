#
# @lc app=leetcode id=144 lang=python3
#
# [144] Binary Tree Preorder Traversal
#
from typing import Optional, List
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
class Solution:
    def preorderTraversal_v1(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        
        left = self.preorderTraversal(root.left)
        right = self.preorderTraversal(root.right)

        return [root.val] + left + right
        
    def preorderTraversal_v2(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        
        stack = [root]
        res = []
        #[5,4,1,2,6,7,8]
        while stack:
            node = stack.pop() # mid first 
            res.append(node.val) # mid 5
            if node.right:
                stack.append(node.right) 
            if node.left:
                stack.append(node.left)
        return res
    
    def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        st = []
        if root:
            st.append(root)
        while st:
            node = st.pop()
            if node != None:
                if node.right:
                    st.append(node.right)
                if node.left:
                    st.append(node.left)
                st.append(node)
                st.append(None)
        
            else:
                node = st.pop()
                res.append(node.val)
        return res
# @lc code=end

