#
# @lc app=leetcode id=24 lang=python3
#
# [24] Swap Nodes in Pairs
#
from typing import Optional
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(next=head)
        cur = dummy

        while cur.next and cur.next.next: # step 1 and step 2
            temp_n = cur.next # 1
            temp_nn = cur.next.next # 2
            temp_nnn = cur.next.next.next # 3

            cur.next = cur.next.next # dummy.next -> 2
            cur.next.next = temp_n # 2 -> 1 
            temp_n.next = temp_nnn # 1 -> 3
            cur = cur.next.next

        # cur = dummy.next
        # while cur:
        #     print(cur.val)
        #     cur = cur.next
        return dummy.next




# @lc code=end
from linked_list.medium_707_design_linked_list import MyLinkedList
l = MyLinkedList()
l.addAtHead(1)
l.addAtTail(2)
l.addAtTail(3)
l.addAtTail(4)
print(l)
s = Solution()
print(s.swapPairs(l.next.next))