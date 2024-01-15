#
# @lc app=leetcode id=242 lang=python3
#
# [242] Valid Anagram
#

# @lc code=start
class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        char_list = [0] * 26
        for i in range(len(s)):
            char_list[ord(s[i]) - ord('a')] += 1
        
        for i in range(len(t)):
            char_list[ord(t[i]) - ord('a')] -= 1
        
        for i in range(len(char_list)):
            if char_list[i] != 0:
                return False
        return True

        
# @lc code=end
