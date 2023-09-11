#
# @lc app=leetcode id=17 lang=python3
#
# [17] Letter Combinations of a Phone Number
#

# @lc code=start
from typing import List


class Solution:
    def __init__(self):
        self.letterMap = [
            "",     # 0
            "",     # 1
            "abc",  # 2
            "def",  # 3
            "ghi",  # 4
            "jkl",  # 5
            "mno",  # 6
            "pqrs", # 7
            "tuv",  # 8
            "wxyz"  # 9
        ]
        self.result = []
        self.string = ""
    def backtracing(self, digits, index):
        if index == len(digits):
            self.result.append(self.string)
            return
        digit = int(digits[index])
        letters = self.letterMap[digit]
        for i in range(len(letters)):
            self.string += letters[i]
            self.backtracing(digits, index + 1)
            self.string = self.string[:-1]

    def letterCombinations(self, digits: str) -> List[str]:
        if len(digits) == 0:
            return self.result
        self.backtracing(digits, 0)
        return self.result
        
# @lc code=end

