#
# @lc app=leetcode id=541 lang=python3
#
# [541] Reverse String II
#

# @lc code=start
from typing import List

class Solution:
    def reverseStr(self, s: str, k: int) -> str:
        def reverseSubstr(substr: List[str]):
            left, right = 0, len(substr) - 1
            while left < right:
                substr[left], substr[right] = substr[right], substr[left]
                left += 1
                right -= 1
            return substr

        s_len = len(s)

        s = list(s)
        for cur in range(0, s_len, 2 * k):
            s[cur: cur + k] = reverseSubstr(s[cur: cur + k])
        
        return ''.join(s)
    
    def replaceSpace(self, s: str) -> str:
        s = list(s)
        raw_len = len(s)
        #  ['w', 'e', ' ', 'a', 'r', 'e', ' ', 'h']
        space_count = 0
        for i in range(raw_len):
            if s[i] == ' ':
                space_count += 1
        
        new_len = raw_len + space_count * (len('%20') - len(' '))
        s.extend([' '] * space_count * (len('%20') - len(' ')))
        #  ['w', 'e', ' ', 'a', 'r', 'e', ' ', 'h', ' ', ' ', ' ', ' ',]
        #  ['w', 'e', ' ', 'a', 'r', 'e', ' ', 'h', ' ', ' ', ' ', ' ',]
        #                                      l|                  r|
        #  ['w', 'e', ' ', 'a', 'r', 'e', ' ', 'h', ' ', ' ', ' ', 'h',]
        #                                      l|                  r|
        #  ['w', 'e', ' ', 'a', 'r', 'e', ' ', 'h', ' ', ' ', ' ', 'h',]
        #                                 l|                  r|
        #  ['w', 'e', ' ', 'a', 'r', 'e', ' ', 'h', '%', '2', '0', 'h',]
        #                                 l|        r|
        left, right = raw_len - 1, new_len - 1
        while left > 0:
            if s[left] == ' ':
                s[right] = '0'
                right -= 1
                s[right] = '2'
                right -= 1
                s[right] = '%'
                right -= 1

            else:
                s[right] = s[left]
                right -= 1

            left -= 1
        
        return ''.join(s)


# @lc code=end

s = Solution()
# print(s.reverseStr("abcdefg", 2))
print(s.replaceSpace("we are h"))