#
# @lc app=leetcode id=1 lang=python3
#
# [1] Two Sum
#
from typing import List
# @lc code=start
class Solution:
    def twoSum_v1(self, nums: List[int], target: int) -> List[int]:
        for i in range(len(nums)):
            for j in range(i+1, len(nums)):
                if nums[i] + nums[j] == target:
                    return[i, j]
                
    def twoSum_v2(self, nums: List[int], target: int) -> List[int]:
        sum_set = set()
        for i, n in enumerate(nums):
            remain = target - n
            if n in sum_set:
                return [nums.index(remain), i]
            sum_set.add(remain)

    def twoSum(self, nums: List[int], target: int) -> List[int]:
        record = dict() # key target-n, v index

        for i, n in enumerate(nums):
            remain = target - n
            if remain in record.keys():
                return [record[remain], i]
            record[n] = i
        
        
# @lc code=end
s = Solution()
print(s.twoSum([2,7,11,15], 9))
