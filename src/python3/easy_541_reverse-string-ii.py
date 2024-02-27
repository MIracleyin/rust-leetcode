# @before-stub-for-debug-begin
from python3problem541 import *
from typing import *
# @before-stub-for-debug-end

#
# @lc app=leetcode id=541 lang=python3
#
# [541] Reverse String II
#
from typing import List
# @lc code=start
class Solution:
    # def reverseStr1(self, s: str, k: int) -> str:

    #     s_len = len(s)
    #     reverse_times = s_len // (2 * k)
    #     remain_len = s_len % (2 * k)
    #     def reverse(s, start, end): # [start, end]
    #         vec_s = list(s)
    #         while start < end:
    #             vec_s[start], vec_s[end] = vec_s[end], vec_s[start]
    #             start += 1
    #             end -= 1
    #         return "".join(vec_s)

    #     for reverse_i in range(reverse_times):
    #         s = reverse(s, reverse_i * 2 * k, reverse_i * 2 * k + k - 1) # [0, 2k]

    #     if remain_len < k:
    #         s = reverse(s, reverse_times * 2 * k, reverse_times * 2 * k + remain_len - 1)
    #     if k <= remain_len < 2 * k:
    #         s = reverse(s, reverse_times * 2 * k, reverse_times * 2 * k + k - 1)
    #     return s

    def reverseStr(self, s: str, k: int) -> str:

        def reverse(sub_str): # [start, end]
            l, r = 0, len(sub_str) - 1
            while l < r:
                sub_str[l], sub_str[r] = sub_str[r], sub_str[l]
                l += 1
                r -= 1
            return sub_str

        new_s = list(s)
        for cur in range(0, len(s), 2 * k):
            new_s[cur: cur + k] = reverse(new_s[cur: cur + k])
        return "".join(new_s)

        # len(left_vec_s) < k -> reverse all
        # k <= len(left_vec_s) < 2k -> reverse (0, k]

# @lc code=end

