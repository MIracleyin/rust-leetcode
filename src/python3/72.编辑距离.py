#
# @lc app=leetcode.cn id=72 lang=python3
#
# [72] 编辑距离
#

# @lc code=start
class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        w1, w2 = len(word1), len(word2)
        
        dp = [[0] * (w2 + 1) for _ in range(w1 + 1)]
        for i in range(w1 + 1):
            dp[i][0] = i # 和当前字符对比需要增加 i 次
        for j in range(w2 + 1):
            dp[0][j] = j

        for i in range(1, w1 + 1):
            for j in range(1, w2 + 1):
                if word1[i - 1] == word2[j - 1]:
                    dp[i][j] = dp[i - 1][j - 1]
                else:
                    dp[i][j] = min(dp[i][j -1], dp[i - 1][j], dp[i - 1][j - 1]) + 1
        return dp[w1][w2]
                    
        
        
        
        
# @lc code=end

