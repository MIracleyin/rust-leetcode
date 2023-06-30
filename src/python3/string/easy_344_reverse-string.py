#
# @lc app=leetcode id=344 lang=python3
#
# [344] Reverse String
#
from typing import List
# @lc code=start
class Solution:
    def reverseString_v1(self, s: List[str]) -> None:
        """
        Do not return anything, modify s in-place instead.
        """
        left, right = 0, len(s) - 1
        while left < right:
            s[left], s[right] = s[right], s[left]
            left += 1
            right -= 1

    def reverseString(self, s: List[str]) -> None:
        """
        Do not return anything, modify s in-place instead.
        """
        stack = []
        for char in s:
            stack.append(char)
        for i in range(len(s)):
            s[i] = stack.pop()
# @lc code=end

s = Solution()
s.reverseString(["h","e","l","l","o"])
