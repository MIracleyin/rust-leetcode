#
# @lc app=leetcode id=347 lang=python3
#
# [347] Top K Frequent Elements
#

from typing import List
# @lc code=start
class Solution:
    
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        import heapq
        map_ = {}
        for i in range(len(nums)):
            map_[nums[i]] = map_.get(nums[i], 0) + 1
        
        pri_que = []

        for num, count in map_.items():
            heapq.heappush(pri_que, (count, num))
            if len(pri_que) > k:
                heapq.heappop(pri_que)
            
        result = [0] * k
        for i in range(k-1, -1, -1):
            result[i] = heapq.heappop(pri_que)[1]
        return result

# @lc code=end
s = Solution()
s.topKFrequent([1,1,1,2,2,3],k=2)
