#
# @lc app=leetcode id=27 lang=python3
#
# [27] Remove Element
#
from typing import List
# @lc code=start
class Solution:
    def removeElement_v1(self, nums: List[int], val: int) -> int:
        # slow, fast = 0, 0
        size = len(nums)
        i = 0
        while i < size:
            if nums[i] == val:
                for j in range(i+1, size):
                    nums[j-1] = nums[j]
                i -= 1
                size -=1
            i += 1
        return size
    def removeElement(self, nums: List[int], val: int) -> int:
        slow, fast = 0, 0
        size = len(nums)
        while fast < size:
            if nums[fast] != val:
                nums[slow] = nums[fast]
                slow += 1
            fast += 1
        return slow


# @lc code=end

s = Solution()
res = s.removeElement([3,2,2,3], 3)
print(res)