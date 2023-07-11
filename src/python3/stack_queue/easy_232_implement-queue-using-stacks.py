#
# @lc app=leetcode id=232 lang=python3
#
# [232] Implement Queue using Stacks
#

# @lc code=start
class MyQueue:

    def __init__(self):
        self.stack_in = []
        self.stack_out = []
        

    def push(self, x: int) -> None:
        self.stack_in.append(x) # push

    def pop(self) -> int:
        if self.empty():
            return None
        
        if self.stack_out:
            return self.stack_out.pop()
        else:
            while self.stack_in:
                self.stack_out.append(self.stack_in.pop())
            return self.stack_out.pop()

    def peek(self) -> int: # no remove
        res = self.pop()
        self.stack_out.append(res)
        return res

    def empty(self) -> bool:
        return not (self.stack_in or self.stack_out)
        


# Your MyQueue object will be instantiated and called as such:
# obj = MyQueue()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.peek()
# param_4 = obj.empty()
# @lc code=end

obj = MyQueue()
obj.push(1) # 1
obj.push(2) # 1 2
print(obj.pop())   # 2
obj.push(3) # 2 3 
obj.push(4) # 2 3 4
print(obj.pop())   # 3 4
print(obj.pop())   # 4  
obj.push(3) # 4 3
obj.push(2) # 4 3 2
obj.push(1) # 4 3 2 1
print(obj.peek())  # 3 2 1
print(obj.peek())  # 2 1
print(obj.peek())  # 1
print(obj.peek())  # None