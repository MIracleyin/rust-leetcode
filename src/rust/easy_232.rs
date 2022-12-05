/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_in.push(x)
    }

    fn pop(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while !self.stack_in.is_empty() {
                self.stack_out.push(self.stack_in.pop().unwrap());
            }
        }
        self.stack_out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        let res = self.pop();
        self.stack_out.push(res);
        res
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

// /**
//  * Your MyQueue object will be instantiated and called as such:
//  * let obj = MyQueue::new();
//  * obj.push(x);
//  * let ret_2: i32 = obj.pop();
//  * let ret_3: i32 = obj.peek();
//  * let ret_4: bool = obj.empty();
//  */
// @lc code=end


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        assert_eq!(queue.empty(), false);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.peek(), 2);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 3);
        assert_eq!(queue.empty(), true);
    }
}