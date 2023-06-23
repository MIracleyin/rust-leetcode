#
# @lc app=leetcode id=202 lang=python3
#
# [202] Happy Number
#
from typing import Union
# @lc code=start
class Solution:
    def isHappy(self, n: int) -> bool:
        record = set()
        while (n != 1):
            n = self.getSum(n)
            if n not in record:
                # print(n)
                record.add(n)
            else:
                return False
        return True
        
        pass
    def getSum(self, n: int) -> int:
        sum = 0
        while(n):
            # sum += (n % 10) ** 2
            # n = n // 10
            n, r = divmod(n, 10)
            sum += r ** 2
        return sum
# @lc code=end
s = Solution()
# print(s.getSum(19))
print(s.isHappy(19))