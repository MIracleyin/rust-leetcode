#
# @lc app=leetcode id=39 lang=python3
#
# [39] Combination Sum
#

# @lc code=start
from typing import List


class Solution:
    def combinationSum_v1(self, candidates: List[int], target: int) -> List[List[int]]:
        results = []
        self.backtracing(candidates, target, 0, 0, [], results)
        return results

    def backtracing_v1(self, candidates, target, total, start_idx, path, results):
        if total > target:
            return

        if total == target:
            results.append(path.copy())
            return
        
        for i in range(start_idx, len(candidates)):
            total += candidates[i]
            path.append(candidates[i])
            self.backtracing(candidates, target, total, i, path, results)
            path.pop()
            total -= candidates[i]

    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        results = []
        candidates.sort()
        self.backtracing(candidates, target, 0, 0, [], results)
        return results

    def backtracing(self, candidates, target, total, start_idx, path, results):
        if total == target:
            results.append(path.copy())
            return
        
        for i in range(start_idx, len(candidates)):
            if total + candidates[i] > target:
                continue
            total += candidates[i]
            path.append(candidates[i])
            self.backtracing(candidates, target, total, i, path, results)
            path.pop()
            total -= candidates[i]    
# @lc code=end

