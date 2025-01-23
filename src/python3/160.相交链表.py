#
# @lc app=leetcode.cn id=160 lang=python3
#
# [160] 相交链表
#
from typing import Optional


class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None
# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def getIntersectionNode1(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        a, b  = headA, headB
        # a: x + z
        # b: y + z
        # x + y + z = x + y + z
        # (x + z) + y = (y + z) + x
        while a is not b: # 刚 node 对不上时，继续遍历 
            a = a.next if a else headB
            b = b.next if b else headA
        return a

    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        a_set = set()
        a, b = headA, headB
        while a:
            a_set.add(a)
            a = a.next
            
        while b:
            if b in a_set:
                return b
            b = b.next
        return None
        
# @lc code=end