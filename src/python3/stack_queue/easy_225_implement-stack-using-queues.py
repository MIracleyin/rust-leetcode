#
# @lc app=leetcode id=225 lang=python3
#
# [225] Implement Stack using Queues
#

# @lc code=start
from collections import deque


class MyStack:

    def __init__(self):
        self.queue = deque()

    def push(self, x: int) -> None:
        self.queue.append(x)
        
    def pop(self) -> int:
        if self.empty():
            return None
        # a, b, c ->  c  b  a -> c
        for _ in range(len(self.queue) - 1):
            self.queue.append(self.queue.popleft())
        return self.queue.popleft()
 
    def top(self) -> int:
        if self.empty():
            return None
        return self.queue[-1]

    def empty(self) -> bool:
        return not self.queue
        


# Your MyStack object will be instantiated and called as such:
# obj = MyStack()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.top()
# param_4 = obj.empty()
# @lc code=end

