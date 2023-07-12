#
# @lc app=leetcode id=1047 lang=python3
#
# [1047] Remove All Adjacent Duplicates In String
#

# @lc code=start
class Solution:
    def removeDuplicates_v1(self, s: str) -> str:
        stack = []
        for char in s:
            if stack and stack[-1] == char:
                stack.pop()
            else:
                stack.append(char)
        return "".join(stack)
    
    def removeDuplicates(self, s: str) -> str:
        res = list(s)
        slow, fast = 0, 0
        lens = len(res)

        while fast < lens:
            res[slow] = res[fast]

            if slow > 0 and res[slow] == res[slow-1]: # 相邻
                slow -= 1
            else:
                slow += 1 # 没有重复就挪
            fast += 1
        return "".join(res[:slow])

# @lc code=end
s = Solution()
print(s.removeDuplicates("abbaca"))