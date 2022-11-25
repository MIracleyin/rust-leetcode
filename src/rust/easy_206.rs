/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */

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
    pub fn reverse_list_v1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut pre = None;
        while let Some(mut node) = cur.take() {
            cur = node.next; // cur point to this node's next node
            node.next = pre; // this node's next point to pre node(first is none)
            pre = Some(node); // pre node is this node
        }
        pre
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn rev(
            mut head: Option<Box<ListNode>>,
            mut pre: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(mut node) = head.take() {
                let cur = node.next;
                node.next = pre;
                pre = Some(node);
                return rev(cur, pre);
            }
            pre
        }
        rev(head, None)
    }
}
// @lc code=end

