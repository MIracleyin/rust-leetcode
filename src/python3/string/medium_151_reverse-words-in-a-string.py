#
# @lc app=leetcode id=151 lang=python3
#
# [151] Reverse Words in a String
#

# @lc code=start
class Solution:
    def reverseWords(self, s: str) -> str:
        #-abc-de-fg-
        s = self.removeSpace(s)
        #abc-de-fg
        s = self.reverseStr(s)
        #gf-ed-cba
        s = list(s)
        start = 0
        for idx in range(len(s)):
            if s[idx] == ' ':
                s[start: idx] = self.reverseStr("".join(s[start: idx])) #fg
                start = idx + 1
        s[start:] = self.reverseStr("".join(s[start:]))
        return ''.join(s)

    def reverseStr(self, s: str) -> str:
        s = list(s)
        left, right = 0, len(s) - 1
        while left < right:
            s[left], s[right] = s[right], s[left]
            left += 1
            right -= 1
        return "".join(s)
    
    def removeSpace(self, s: str, element=' ') -> str:
        s = list(s)
        fast, slow = 0, 0
        size = len(s)
        #  f|
        # [' ', 'a', 'b', ' ', 'c', ' ', ' ', 'f', 'g', ' ']
        #  s|
        #       f|
        # ['a', 'a', 'b', ' ', 'c', ' ', ' ', 'f', 'g', ' ']
        #  s|
        #            f|
        # ['a', 'b', 'b', ' ', 'c', ' ', ' ', 'f', 'g', ' ']
        #        s|
        #                 f|
        # ['a', 'b', 'b', ' ', 'c', ' ', ' ', 'f', 'g', ' ']
        #             s|
        #                      f|
        # ['a', 'b', 'b', ' ', 'c', ' ', ' ', 'f', 'g', ' ']
        #             s|
        #                      f|
        # ['a', 'b', ' ', ' ', 'c', ' ', ' ', 'f', 'g', ' ']
        #                 s|
        #                      f|
        # ['a', 'b', ' ', 'c', 'c', ' ', ' ', 'f', 'g', ' ']
        #                 s|
        #                                     f|
        # ['a', 'b', ' ', 'c', 'c', ' ', ' ', 'f', 'g', ' ']
        #                      s|
        #                                               f|
        # ['a', 'b', ' ', 'c', ' ', 'f', 'g', 'f', 'g', ' ']
        #                                     s|
        while fast < size:
            if s[fast] != element: # 如果 fast 有元素
                if slow != 0: # 且不在第一位
                    s[slow] = element
                    slow += 1
                while fast < size and s[fast] != element:
                    s[slow] = s[fast]
                    slow += 1
                    fast += 1

            fast += 1
        return "".join(s[:slow])
    


# @lc code=end
s = Solution()
print(s.reverseStr("abc"))
print(s.removeSpace(" abc de    fg "))
print(s.reverseWords(" abc  de fg "))
print(s.reverseWords("the sky is blue"))

