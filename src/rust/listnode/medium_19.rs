/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let new_node = Box::new(ListNode { val, next: None });
        let mut cur = &mut self.next;
        while let Some(node) = cur {
            cur = &mut node.next;
        }
        *cur = Some(new_node)
    }
}

struct Solution;

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_nth_from_end_v1(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;
        let mut fast = &dummy_head.clone();
        let mut slow = &mut dummy_head;

        while n > 0 {
            fast = fast.next.as_ref().unwrap(); // fast move n 
            n -= 1;
        }

        while fast.next.is_some() { // not is none
            // fast and slow move len - n
            fast = fast.next.as_ref().unwrap();
            slow = slow.next.as_mut().unwrap();
        }
        slow.next = slow.next.as_mut().unwrap().next.take();
        dummy_head.next
    }

    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        Self::travel(&mut dummy, n);
        dummy.next
    }

    pub fn travel(cur: &mut Box<ListNode>, n: i32) -> i32 {
        if let Some(node) = cur.next.as_mut() {
            let num = 1 + Self::travel(node, n);
            // println!("{}", num);
            // println!("{:#?}", node);
            if num == n {
                cur.next = node.next.take();
            }
            return num;
        }

        0
    }
}
// @lc code=end

mod test {
    use super::Solution;
    use crate::rust::listnode::{medium_707, medium_19::*};

    #[test]
    fn it_works() {
        let mut head = ListNode::new(0);
        head.add_at_tail(1);
        head.add_at_tail(2);
        head.add_at_tail(3);
        head.add_at_tail(4);
        head.add_at_tail(5);

        // println!("{:#?}",head);

        let new_head = Solution::remove_nth_from_end(head.next, 2);

        // println!("{:#?}", new_head)
    }
}
