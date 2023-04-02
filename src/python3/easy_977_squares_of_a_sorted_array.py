#
# @lc app=leetcode id=977 lang=python3
#
# [977] Squares of a Sorted Array
#
from typing import List
# @lc code=start
class Solution:
    def sortedSquares_v1(self, nums: List[int]) -> List[int]:
        nums = [n*n for n in nums] # n
        return sorted(nums) # nlogn
    
    def sortedSquares(self, nums: List[int]) -> List[int]:
        n = len(nums)
        i, j, k = 0, n - 1, n - 1
        res = [-1] * n
        while i <= j:
            if nums[i] * nums[i] > nums[j] * nums[j]:
                res[k] = nums[i] * nums[i]
                i += 1
            else:
                res[k] = nums[j] * nums[j]
                j -= 1
            k -= 1
        return res

# @lc code=end
ret = Solution().sortedSquares([-4,-1,0,3,10])
print(ret)

