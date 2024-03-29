#
# @lc app=leetcode id=94 lang=python3
#
# [94] Binary Tree Inorder Traversal
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
    def inorderTraversal_v1(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        
        left = self.inorderTraversal(root.left)
        right = self.inorderTraversal(root.right)

        return left + [root.val] + right
    
    def inorderTraversal_v2(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        
        stack = []
        res = []
        cur = root
        # 1 4 2 5 6 7 8 
        while cur or stack:
            if cur:
                stack.append(cur)
                cur = cur.left
            else:
                cur = stack.pop()
                res.append(cur.val)
                cur = cur.right
        return res
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        st = []
        if root:
            st.append(root)
        while st:
            node = st.pop()
            if node != None:
                if node.right:
                    st.append(node.right)

                st.append(node)
                st.append(None)
                
                if node.left:
                    st.append(node.left)
            else:
                node = st.pop()
                res.append(node.val)
        return res
# @lc code=end

