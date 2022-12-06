/*
 * @lc app=leetcode id=225 lang=rust
 *
 * [225] Implement Stack using Queues
 */

// @lc code=start
struct MyStack_v1 {
    que1: Vec<i32>,
    que2: Vec<i32>,
}

struct MyStack {
    que: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack_v1 {

    fn new() -> Self {
        MyStack_v1 {
            que1: Vec::new(),
            que2: Vec::new(),
        }
    }
    fn push(&mut self, x: i32) {
        self.que1.push(x)
    }
    
    fn pop(&mut self) -> i32 {
        let len = self.que1.len() - 1;
        for _ in 0..len {
            self.que2.push(self.que1.remove(0));
        }

        let res = self.que1.remove(0);

        while !self.que2.is_empty() {
            self.que1.push(self.que2.remove(0))
        }

        res
    }
    
    fn top(&mut self) -> i32 {
        let res = self.pop();
        self.que1.push(res);
        res
    }
    
    fn empty(&self) -> bool {
        self.que1.is_empty()
    }
}

impl MyStack {
    fn new() -> Self{
        MyStack {
            que: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.que.push(x)
    }

    fn pop(&mut self) -> i32 {
        let len = self.que.len() - 1;
        for _ in 0..len {
            let tmp = self.que.remove(0);
            self.que.push(tmp);
        }
        self.que.remove(0)

    }

    fn top(&mut self) -> i32 {
        let res = self.pop();
        self.que.push(res);
        res
    }

    fn empty(&self) -> bool {
        self.que.is_empty()
    }
}

// /**
//  * Your MyStack object will be instantiated and called as such:
//  * let obj = MyStack::new();
//  * obj.push(x);
//  * let ret_2: i32 = obj.pop();
//  * let ret_3: i32 = obj.top();
//  * let ret_4: bool = obj.empty();
//  */
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue = MyStack::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        // assert_eq!(queue.empty(), false); 
        assert_eq!(queue.pop(), 3);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 1);
        // assert_eq!(queue.empty(), true);
    }
}
