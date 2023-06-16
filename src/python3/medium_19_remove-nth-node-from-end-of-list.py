#
# @lc app=leetcode id=19 lang=python3
#
# [19] Remove Nth Node From End of List
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
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy = ListNode(next=head)
        fast, slow = dummy, dummy
        # 1. move fast to n
        # 2. move fast and slow both, until fast to the end
        # 3. remove slow, slow.next = slow.next.next
        while n > 0:
            fast = fast.next
            n -= 1
        
        while fast.next:
            fast = fast.next
            slow = slow.next

        temp_s = slow.next
        slow.next = temp_s.next

        return dummy.next


        

        
# @lc code=end
from linked_list.medium_707_design_linked_list import MyLinkedList
l = MyLinkedList()
l.addAtHead(1)
l.addAtTail(2)
l.addAtTail(3)
l.addAtTail(4)
l.addAtTail(5)
print(l)
s = Solution()
print(s.removeNthFromEnd(l.next.next, 2))
