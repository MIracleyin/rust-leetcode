#
# @lc app=leetcode.cn id=541 lang=python3
#
# [541] 反转字符串 II
#

# @lc code=start
class Solution:
    def reverseStr(self, s: str, k: int) -> str:
        def reverse(s) -> str:
            l, r = 0, len(s) - 1
            while l < r:
                s[l], s[r] = s[r], s[l]
                l += 1
                r -= 1
            return s
        
        res = list(s)
        for cur in range(0, len(s), 2 * k):
            res[cur: cur + k] = reverse(res[cur: cur + k])
        
        return ''.join(res)
        
# @lc code=end

