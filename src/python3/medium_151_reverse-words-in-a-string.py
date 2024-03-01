# @before-stub-for-debug-begin
from python3problem151 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=151 lang=python3
#
# [151] Reverse Words in a String
#
from typing import List
# @lc code=start
class Solution:
    def reverseWords(self, s: str) -> str:
        s = self.removeExtraSpaces(s)
        s = self.reverse(s, 0, len(s) - 1)

        start = 0
        s_list = list(s)
        for i in range(len(s_list) + 1):
            if i == len(s_list) or s_list[i] == ' ':
                s = self.reverse(s_list, start, i - 1)
                start = i + 1
        return s
    def removeExtraSpaces(self, s):
        # 去除所有空格并在相邻单词之间添加空格, 快慢指针。
        slow = 0  # 整体思想参考https://programmercarl.com/0027.移除元素.html
        s_list = list(s)
        word_started = False  # 标记是否已经开始处理单词
        for i in range(len(s_list)):
            if s_list[i] != ' ':
                if word_started and s_list[slow - 1] != ' ':
                    s_list.append(' ')
                    slow += 1
                s_list.append(s_list[i])
                slow += 1
                word_started = True
            else:
                word_started = False
        return ''.join(s_list[:slow])

    def reverse(self, s: str, start: int, end: int) -> str:
        s_list = list(s)
        while start < end:
            s_list[start], s_list[end] = s_list[end], s_list[start]
            start += 1
            end -= 1
        return ''.join(s_list)

        # words = s.split()

        # left, right = 0, len(words) - 1
        # while left < right:
        #     words[left], words[right] = words[right], words[left]
        #     left += 1
        #     right -= 1
        
        # return " ".join(words)


# @lc code=end


s = Solution()
res = s.reverseWords("the sky is blue")
print(res)
