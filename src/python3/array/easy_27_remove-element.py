# @before-stub-for-debug-begin
from python3problem27 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=27 lang=python3
#
# [27] Remove Element
#

# @lc code=start
from typing import List


class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        slow = 0
        for fast in range(len(nums)):
            if nums[fast] != val:
                nums[slow] = nums[fast]
                slow += 1
                
        return slow 

        
# @lc code=end

