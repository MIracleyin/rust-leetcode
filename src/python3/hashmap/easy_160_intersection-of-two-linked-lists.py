#
# @lc app=leetcode id=160 lang=python3
#
# [160] Intersection of Two Linked Lists
#
from typing import Optional
class ListNode:
    def __init__(self, x=0, next=None):
        self.val = x
        self.next = next
# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

def print_list(head: ListNode):
    dummy = ListNode(next=head)
    cur = dummy.next
    while cur:
        print(cur.val)
        cur = cur.next

class Solution:
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        # 获取 List 长度
        curA = headA
        curB = headB

        len_a, len_b = 1, 1
        while curA.next:
            curA = curA.next
            len_a += 1

        while curB.next:
            curB = curB.next
            len_b += 1
        
        curA, curB = headA, headB
        if len_a > len_b:
            curA, curB = headB, headA
            len_a, len_b = len_b, len_a

        # 按照尾节点对齐 A B List
        for _ in range(len_b - len_a):
            curB = curB.next

        # 遍历直到发现相同节点 
        while curA:
            if curA.val == curB.val: # why ac, but can't succees in local
                print(curA.val)
                print(curA)
                print(curB.val)
                print(curB)
                return curA
            else:
                curA, curB = curA.next, curB.next
        return None

# @lc code=end

from linked_list.medium_707_design_linked_list import MyLinkedList
a = MyLinkedList()
a.addAtHead(4)
a.addAtTail(1)
a.addAtTail(8)
a.addAtTail(4)
a.addAtTail(5)
b = MyLinkedList()
b.addAtHead(5)
b.addAtTail(6)
b.addAtTail(1)
b.addAtTail(8)
b.addAtTail(4)
b.addAtTail(5)
s = Solution()
print_list(a)
# print(s.getIntersectionNode(a.next.next, b.next.next))
