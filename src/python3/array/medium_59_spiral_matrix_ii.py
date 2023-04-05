#
# @lc app=leetcode id=59 lang=python3
#
# [59] Spiral Matrix II
#
from typing import List
# @lc code=start
class Solution:
    def generateMatrix(self, n: int) -> List[List[int]]:
        nums = [[0] * n for _ in range(n)]
        start_x, start_y = 0, 0
        loop, mid = n // 2, n // 2
        count = 1

        for offset in range(1, loop + 1):
            for i in range(start_y, n - offset):
                nums[start_x][i] = count
                count += 1
            for i in range(start_x, n - offset):
                nums[i][n - offset] = count
                count += 1
            for i in range(n - offset, start_y, -1):
                nums[n - offset][i] = count
                count += 1
            for i in range(n - offset, start_x, -1):
                nums[i][start_y] = count
                count += 1
            start_x += 1
            start_y += 1



        if n % 2 != 0:
            nums[mid][mid] = count
        
        return nums

# @lc code=end

ret = Solution().generateMatrix(4)