#
# @lc app=leetcode.cn id=704 lang=python3
#
# [704] 二分查找
#

# @lc code=start
from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        n = len(nums)
        l, r = 0 , n - 1

        while l <= r:
            mid = (l + r) // 2
            if nums[mid] < target:
                l = mid + 1
            elif nums[mid] > target:
                r = mid - 1
            else: # find
                return mid
        return -1
        
# @lc code=end

csighub.tencentyun.com/ti-ocr-docker/tiocr_ydet:ppl-a7d3c1b9-23b9-40d7-a075-5e2c48c4e606