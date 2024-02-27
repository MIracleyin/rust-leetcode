# @before-stub-for-debug-begin
from python3problem18 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=18 lang=python3
#
# [18] 4Sum
#
from typing import List
# @lc code=start
class Solution:
    def fourSum(self, nums: List[int], target: int) -> List[List[int]]:
        result = []
        nums.sort()
        len_nums = len(nums)

        # if nums[0] > target:
        #     return result
        
        for i in range(len_nums):

            # if i > 0 and nums[i] > target: continue

            if i > 0 and nums[i] == nums[i - 1]: continue

            for j in range(i + 1, len_nums):

                if j > i + 1 and nums[j] == nums[j - 1]: continue
                
                left, right = j + 1, len_nums - 1
                while right > left:
                    sum_ = nums[i] + nums[j] + nums[left] + nums[right]
                    if sum_ < target:
                        left += 1
                    elif sum_ > target:
                        right -= 1
                    else:
                        result.append([nums[i], nums[j], nums[left], nums[right]])

                        while right > left and nums[left] == nums[left + 1]: left += 1
                        while right > left and nums[right] == nums[right - 1]: right -= 1

                        left += 1
                        right -= 1
            
        return result

        
# @lc code=end

