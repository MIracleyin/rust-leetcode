# @before-stub-for-debug-begin
from python3problem977 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=977 lang=python3
#
# [977] Squares of a Sorted Array
#

# @lc code=start
from typing import List


class Solution:
    def sortedSquares(self, nums: List[int]) -> List[int]:
        left, right = 0, len(nums) - 1
        k = len(nums) - 1
        squa_num = [0] * len(nums)
        while k >= 0:
            if nums[left] ** 2 > nums[right] ** 2:
                squa_num[k] = nums[left] ** 2
                left += 1
            else:
                squa_num[k] = nums[right] ** 2
                right -= 1
            k -= 1
        return squa_num
        
# @lc code=end

