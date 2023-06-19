#
# @lc app=leetcode id=242 lang=python3
#
# [242] Valid Anagram
#

# @lc code=start
class Solution:
    def isAnagram_v1(self, s: str, t: str) -> bool:
        record = [0] * 26
        for si in s:
            record[ord(si) - ord('a')] += 1
        
        for ti in t:
            record[ord(ti) - ord('a')] -= 1

        # if sum(record) != 0: # 不能保证全 0
        #     return False
        for r in record:
            if r != 0:
                return False
        
        return True
# @lc code=end

s = Solution()
r = s.isAnagram("rat", "car")
print(r)

