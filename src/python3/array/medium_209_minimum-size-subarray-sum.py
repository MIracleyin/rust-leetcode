# @before-stub-for-debug-begin
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=209 lang=python3
#
# [209] Minimum Size Subarray Sum
#

# @lc code=start
from typing import List


class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        left, right = 0, 0
        size = len(nums)
        min_len = float('inf')
        cur_sum = 0
        while right < size:
            cur_sum += nums[right]
            
            while cur_sum >= target:
                min_len = min(min_len, right - left + 1)
                cur_sum -= nums[left]
                left += 1
            
            right += 1
        if min_len == float('inf'):
            return 0
        else:
            return min_len
            
            


        
# @lc code=end

