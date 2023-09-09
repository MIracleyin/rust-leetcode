# @before-stub-for-debug-begin
from python3problem77 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=77 lang=python3
#
# [77] Combinations
#

# @lc code=start
from typing import List


class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        results = []
        self.backtracking(n, k, 1, [], results)
        return results

    def backtracking_v1(self, n: int, k: int, start_idx: int, path, results):
        if len(path) == k:
            results.append(path)
            return
        for i in range(start_idx, n + 1):  # start_idx, 5 # range need 4
            path.append(i)
            self.backtracking(n, k, i + 1, path, results)
            path.pop()

    def backtracking(self, n: int, k: int, start_idx: int, path, results):
        if len(path) == k:
            results.append(path.copy()) # path will change, so need a copy here
            return
        
        for i in range(start_idx, (n + 1) - (k - len(path)) + 1):
            path.append(i)
            self.backtracking(n, k, i + 1, path, results)
            path.pop()

        
        


        
# @lc code=end

