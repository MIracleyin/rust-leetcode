# @before-stub-for-debug-begin
from python3problem15 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=15 lang=python3
#
# [15] 3Sum
#
from typing import List
# @lc code=start

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        result = []
        nums.sort()

        for i in range(len(nums)):
            if nums[i] > 0:
                return result
            
            if i > 0 and nums[i] == nums[i - 1]: # i > 0 ?
                continue
            
            left, right = i + 1, len(nums) - 1
            while right > left:
                sum_triplet = nums[i] + nums[left] +nums[right]
                if sum_triplet < 0:
                    left += 1
                elif sum_triplet > 0:
                    right -= 1
                else:
                    result.append([nums[i], nums[left], nums[right]])

                    while right > left and nums[right] == nums[right - 1]:
                        right -= 1
                    while right > left and nums[left] == nums[left + 1]:
                        left += 1

                    left += 1
                    right -= 1


        
        return result


        
# @lc code=end

