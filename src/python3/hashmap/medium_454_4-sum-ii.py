#
# @lc app=leetcode id=454 lang=python3
#
# [454] 4Sum II
#
from typing import List
# @lc code=start
class Solution:
    def fourSumCount(self, nums1: List[int], nums2: List[int], nums3: List[int], nums4: List[int]) -> int:
        n1_n2_record = dict()
        for n1 in nums1:
            for n2 in nums2:
                if n1 + n2 in n1_n2_record:
                    n1_n2_record[n1 + n2] += 1
                else:
                    n1_n2_record[n1 + n2] = 1

        count = 0
        for n3 in nums3:
            for n4 in nums4:
                key = - n3 - n4
                if key in n1_n2_record:
                    count += n1_n2_record[key]

        return count

# @lc code=end

