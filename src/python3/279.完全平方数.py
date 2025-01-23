#
# @lc app=leetcode.cn id=279 lang=python3
#
# [279] 完全平方数
#

# @lc code=start
class Solution:
    def numSquares1(self, n: int) -> int:
        nums = [i*i for i in range(1, int(n**0.5) + 1)]
        dp = [0] + [float('inf')] * n
        for num in nums:
            for j in range(num, n + 1):
                dp[j] = min(dp[j], dp[j - num] + 1)
        return dp[-1]

    def numSquares(self, n: int) -> int:
        ps = set([i*i for i in range(1, int(n**0.5) + 1)])
        def divisible(n, conut):
            if conut == 1:
                return n in ps
            for p in ps:
                if divisible(n - p, conut - 1):
                    return True
            return False

        for conut in range(1, n + 1):
            if divisible(n, conut):
                return conut
        
# @lc code=end

