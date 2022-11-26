/*
 * @lc app=leetcode.cn id=707 lang=rust
 *
 * [707] 设计链表
 */

// @lc code=start
#[derive(Debug)]
pub struct MyLinkedList {
    pub val: i32,
    pub next: Option<Box<MyLinkedList>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    pub fn new(val: i32) -> Self {
        MyLinkedList { val, next: None }
    }

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }
        let mut n = 0;
        let mut cur = &self.next; // head point

        while let Some(node) = cur {
            if n == index {
                return node.val;
            }
            n += 1;
            cur = &node.next; // point to next node
        }
        // over
        -1
    }

    pub fn add_at_head(&mut self, val: i32) {
        let new_node = Box::new(MyLinkedList {
            val,
            next: self.next.take(),
        });

        self.next = Some(new_node);
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let new_node = Box::new(MyLinkedList { val, next: None });
        let mut cur = &mut self.next;
        while let Some(node) = cur {
            cur = &mut node.next;
        }
        *cur = Some(new_node)
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index <= 0 {
            self.add_at_head(val);
        } else {
            let mut n = 0;
            let mut cur = &mut self.next;
            while let Some(node) = cur {
                if n + 1 == index {
                    // index is cur next
                    let new_node = Box::new(MyLinkedList {
                        val,
                        next: node.next.take(),
                    });
                    node.next = Some(new_node);
                    break;
                }
                n += 1;
                cur = &mut node.next;
            }
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }

        let mut n = 0;
        let mut cur = self;
        while let Some(node) = cur.next.take() {
            if n == index {
                cur.next = node.next;
                break;
            }
            n += 1;
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
    }
}

// /**
//  * Your MyLinkedList object will be instantiated and called as such:
//  * let obj = MyLinkedList::new();
//  * let ret_1: i32 = obj.get(index);
//  * obj.add_at_head(val);
//  * obj.add_at_tail(val);
//  * obj.add_at_index(index, val);
//  * obj.delete_at_index(index);
//  */
// @lc code=end
