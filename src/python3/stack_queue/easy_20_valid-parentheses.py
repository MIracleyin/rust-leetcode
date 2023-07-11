#
# @lc app=leetcode id=20 lang=python3
#
# [20] Valid Parentheses
#

# @lc code=start
class Solution:
    def isValid(self, s: str) -> bool:
        stack = []

        for item in s:
            if item == '(':
                stack.append(')')
            elif item == '[':
                stack.append(']')
            elif item == '{':
                stack.append('}') # ([{
            elif not stack or stack[-1] != item:
                return False # 只能判断正确顺序闭合，不能判断不正确
            else:
                stack.pop()
        return True if not stack else False
    # def isValid(self, s: str) -> bool:
    #     stack = []

    #     for item in s:
    #         match item:
    #             case '(':
    #                 stack.append(')')
    #             case '[':
    #                 stack.append(']')
    #             case '{':
    #                 stack.append('}')
    #             case stack[-1]
    #                 stack.pop()
    #     return True if not stack else False
    
    # def isValid2(self, s: str) -> bool:
    #     stack = []

    #     for item in s:
    #         if item == '(':
    #             stack.append(')')
    #         elif item == '[':
    #             stack.append(']')
    #         elif item == '{':
    #             stack.append('}') # ([{
    #         elif not stack or item not in stack:
    #             return False # 只能判断正确顺序闭合，不能判断不正确
    #         else:
    #             stack.remove(item)
    #     return True if not stack else False
    
# @lc code=end

v = "{[({[(}])}])"
s = Solution()
print(s.isValid2(v))

