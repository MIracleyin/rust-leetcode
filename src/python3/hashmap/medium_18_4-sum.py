#
# @lc app=leetcode id=18 lang=python3
#
# [18] 4Sum
#
from typing import List
# @lc code=start
class Solution:
    def fourSum(self, nums: List[int], target: int) -> List[List[int]]:
        nums = sorted(nums)
        n = len(nums)
        result = []
        for i in range(n): # if target is a really small number
            if nums[i] > target and nums[i] > 0 and target > 0: # 1 cut 
                break
            if i > 0 and nums[i] == nums[i-1]: # 1 dededuplicate
                continue
            for j in range(i+1, n):
                if nums[i] + nums[j] > target and target > 0: # 2 cut
                    break
                if j > i + 1 and nums[j] == nums[j-1]: # 2 dededuplicate
                    continue
                left, right = j + 1, n - 1
                while left < right:
                    sum = nums[i] + nums[j] + nums[left] + nums[right]
                    if sum < target:
                        left += 1
                    elif sum > target:
                        right -= 1
                    else: # s == target
                        result.append([nums[i], nums[j], nums[left], nums[right]])
                        while left < right and nums[left] == nums[left+1]:
                            left += 1
                        while left < right and nums[right] == nums[right-1]:
                            right -= 1
                        left += 1
                        right -= 1
        return result
# @lc code=end
s = Solution()
print(s.fourSum([-2,-1,-1,1,1,2,2], 0))