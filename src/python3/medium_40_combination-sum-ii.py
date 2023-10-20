#
# @lc app=leetcode id=40 lang=python3
#
# [40] Combination Sum II
#
from typing import List
# @lc code=start

class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        result = []
        candidates.sort()
        self.backtracking(candidates, target, 0, 0, [], result)
        return result

    def backtracking(self, candidates, target, totoal, start_idx, path, result):
        if totoal == target:
            result.append(path[:])
            return
        
        for i in range(start_idx, len(candidates)):
            if i > start_idx and candidates[i] == candidates[i - 1]:
                continue

            if totoal + candidates[i] > target:
                break

            path.append(candidates[i])
            totoal += candidates[i]
            self.backtracking(candidates, target, totoal, i + 1, path, result)
            totoal -= candidates[i]
            path.pop()
        
# @lc code=end

