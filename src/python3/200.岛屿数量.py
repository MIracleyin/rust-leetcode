#
# @lc app=leetcode.cn id=200 lang=python3
#
# [200] 岛屿数量
#

# @lc code=start
from typing import List


class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m, n = len(grid), len(grid[0])
        
        def dfs(row: int, col: int):
            # 取不到等，防止溢出
            if row < 0 or row >= m or col < 0 or col >= n or grid[row][col] != "1":
                return 

            grid[row][col] = '2'
            dfs(row, col - 1) # 左
            dfs(row, col + 1) # 右
            dfs(row - 1, col) # 上
            dfs(row + 1, col) # 下        
        
        ans = 0
        for row_idx in range(m):
            row = grid[row_idx]
            for col_idx in range(n):
                cur = row[col_idx]
                if cur == "1":
                    # mark island
                    dfs(row_idx, col_idx)
                    ans += 1
        
        return ans
        
# @lc code=end

