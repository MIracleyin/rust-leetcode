#
# @lc app=leetcode id=150 lang=python3
#
# [150] Evaluate Reverse Polish Notation
#
from typing import List
# @lc code=start
class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        from operator import add, sub, mul
        op_map = {'+': add, '-': sub, '*': mul, '/': lambda x, y: int(x / y)}
        stack = []
        for token in tokens:
            if token not in op_map.keys(): # number
                stack.append(int(token))
            else: # ops
                op2 = stack.pop()
                op1 = stack.pop()
                stack.append(op_map[token](op1, op2)) # cal and save
        return stack.pop()
# @lc code=end
s = Solution()
s.evalRPN(["2","1","+","3","*"])
