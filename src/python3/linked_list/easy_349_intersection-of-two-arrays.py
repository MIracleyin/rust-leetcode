#
# @lc app=leetcode id=349 lang=python3
#
# [349] Intersection of Two Arrays
#
from typing import List
# @lc code=start
class Solution:
    def intersection_v1(self, nums1: List[int], nums2: List[int]) -> List[int]:
        table = {}
        for n1 in nums1:
            table[n1] = table.get(n1, 0) + 1

        res = set()
        for n2 in nums2:
            if n2 in table.keys():
                res.add(n2)
                del table[n2]
                
        return list(res)
    
    def intersection(self, nums1: List[int], nums2: List[int]) -> List[int]:
        return list(set(nums1) & set(nums2))
# @lc code=end

