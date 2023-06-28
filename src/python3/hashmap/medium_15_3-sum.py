#
# @lc app=leetcode id=15 lang=python3
#
# [15] 3Sum
#
from typing import List
# @lc code=start
class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums = sorted(nums) # sort small -> large
        result = []
        for i in range(len(nums)):
            if nums[i] > 0:
                return result
            
            # 跳过相同的元素以避免重复
            if i > 0 and nums[i] == nums[i - 1]: # 如果上一个元素计算过
                continue

            left, right = i + 1, len(nums) - 1
            while left < right:
                sum = nums[i] + nums[left] + nums[right]
                if sum < 0:
                    left += 1
                elif sum > 0:
                    right -= 1
                else:
                    result.append([nums[i], nums[left], nums[right]])
                    # 跳过相同的元素以避免重复
                    while right > left and nums[right] == nums[right - 1]: # 如果下一个元素相同
                        right -= 1
                    while right > left and nums[left] == nums[left + 1]: # 如果下一个元素相同
                        left += 1
                        
                    right -= 1
                    left += 1

        return result
 
# @lc code=end

s = Solution()
print(s.threeSum([-1,0,1,2,-1,-4]))