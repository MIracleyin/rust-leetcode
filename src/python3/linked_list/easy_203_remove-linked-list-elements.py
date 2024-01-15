#
# @lc app=leetcode id=203 lang=python3
#
# [203] Remove Linked List Elements
#
from typing import Optional

from python3.linked_list.easy_203_remove_linked_list_elements import ListNode

# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next


class Solution:
    def removeElements(self, head: Optional[ListNode], val: int) -> Optional[ListNode]:
        dummy_head = ListNode(next=head)
        cur_node = dummy_head
        while cur_node.next != None:
            if cur_node.next.val == val:
                cur_node.next = cur_node.next.next
            else:
                cur_node = cur_node.next
        return dummy_head.next
    

        
# @lc code=end

