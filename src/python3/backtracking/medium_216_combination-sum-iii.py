#
# @lc app=leetcode id=216 lang=python3
#
# [216] Combination Sum III
#

# @lc code=start
from typing import List


class Solution:
    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        path = []
        results = []
        self.backtracing(k, n, 0, 0 + 1, path, results)
        return results
    def backtracing(self, k: int, target_n: int, current_n: int, start_i, path, results):
        if current_n > target_n:
            return

        if len(path) == k:
            if current_n == target_n:
                results.append(path.copy())
            return
        
        for i in range(start_i, (9 + 1) - (k - len(path)) + 1):
            current_n += i
            path.append(i)
            self.backtracing(k, target_n, current_n, i + 1, path, results)
            current_n -= i
            path.pop()



        
# @lc code=end

