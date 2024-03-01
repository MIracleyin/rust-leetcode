# @before-stub-for-debug-begin
from python3problem88 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=88 lang=python3
#
# [88] Merge Sorted Array
#
from typing import List
# @lc code=start

class Solution:
    def merge_old(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        if n == 0: return
        len1 = len(nums1)
        end_idx = len1 - 1
        while n > 0 and m > 0:
            if nums2[n-1] >= nums1[m-1]:
                nums1[end_idx] = nums2[n-1]
                n -= 1
            else:
                nums1[end_idx] = nums1[m-1]
                m -= 1
            end_idx -= 1
        while n > 0:
            nums1[end_idx] = nums2[n-1]
            n -= 1
            end_idx -= 1

    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        if n == 0: return # 如果 nums2 为空，nums1 为结果
        len1 = len(nums1) # 得到最终数组长度
        end_idx = len1 - 1 # 得到最终数组最后一位 idx

        # 消耗两个数组
        while n > 0 and m > 0:
            if nums2[n - 1] >= nums1[m - 1]: # n-1, m-1 表示当前数组有效数字最后一位
                nums1[end_idx] = nums2[n - 1]
                n -= 1
            else:
                nums1[end_idx] = nums1[m - 1]
                m -= 1
            end_idx -= 1
        
        # 如果 nums1 将后面的空填满，但是 nums2 还有剩余，（不可能出现 nums2 填满，nums1 有空余，nums1 至少可以保留原位）
        while n > 0:
            nums1[end_idx] = nums2[n - 1]
            n -= 1
            end_idx -= 1
        


# @lc code=end

