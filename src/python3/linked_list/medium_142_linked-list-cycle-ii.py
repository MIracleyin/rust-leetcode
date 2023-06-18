#
# @lc app=leetcode id=142 lang=python3
#
# [142] Linked List Cycle II
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
    def detectCycle_v1(self, head: Optional[ListNode]) -> Optional[ListNode]:
        cur = head
        visited = set()
        while cur:
            # if cur in vis
            if cur in visited:
                return cur
            # else add cur
            visited.add(cur)
            cur = cur.next

        return None
    
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        fast, slow = head, head
        # search meet
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
            if slow == fast: # if them will meet
                slow = head
                while slow != fast: # until them are same
                    slow = slow.next
                    fast = fast.next
                return slow
        pass
        
# @lc code=end

