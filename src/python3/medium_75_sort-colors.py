# @before-stub-for-debug-begin
from python3problem75 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=75 lang=python3
#
# [75] Sort Colors
#
from typing import List
# @lc code=start
class Solution:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        # (0, red] (red+1, while] (while +1, len(nums) - 1]
        # left -> index_red right -> index_while
        # red:[0, red) white:[red, i),[i, blue) [blue, n)
        # 
        n = len(nums)
        r, i, b = 0, 0, n #[0, r), i, [b, n)
        while i < b:
            if nums[i] == 0:
                nums[r], nums[i] = nums[i], nums[r]
                r += 1
                i += 1 # 符合位置
            elif nums[i] == 2:
                b -= 1
                # i += 1 后面多了一位，但当前位置依旧需要判断
                nums[b], nums[i] = nums[i], nums[b]
            else:
                i += 1 # 符合位置
        
# @lc code=end

