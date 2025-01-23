#
# @lc app=leetcode.cn id=121 lang=python3
#
# [121] 买卖股票的最佳时机
#

# @lc code=start
from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        min_price = float('inf')
        max_profit = 0
        
        for i in range(len(prices)):
            max_profit = max(prices[i] - min_price, max_profit)
            min_price = min(prices[i], min_price)
        
        return max_profit
# @lc code=end

