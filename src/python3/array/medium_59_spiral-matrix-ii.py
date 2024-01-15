# @before-stub-for-debug-begin
from python3problem59 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=59 lang=python3
#
# [59] Spiral Matrix II
#

# @lc code=start
from typing import List


class Solution:
    def generateMatrix(self, n: int) -> List[List[int]]:
        res = [[0] * n for _ in range(n)]
        start_x, start_y = 0, 0
        loop, mid = n // 2, n // 2
        offset = 1
        count = 1
        while loop > 0:
            i, j = start_x, start_y
            
            # 0, 0 -> 0, n-1
            for j in range(start_y, n - offset): # (0, 3 - 1) (0, 2), (0, 1)
                res[start_x][j] = count
                count += 1
            
            # 0, n -> n-1, n
            for i in range(start_x, n - offset):
                res[i][n - offset] = count
                count += 1
            
            # n, n -> 0, n - 1
            for j in range(n - offset, start_y, -1):
                res[n - offset][j] = count
                count += 1

            # 0, n -> 0, 0
            for i in range(n - offset, start_x, -1):
                res[i][start_y] = count
                count += 1
            
            start_x += 1
            start_y += 1
            offset += 1
            loop -= 1
        
        if n % 2 != 0:
            res[mid][mid] = n * n 
        
        return res
            
# @lc code=end

