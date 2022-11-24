// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
/*
 * @lc app=leetcode.cn id=203 lang=rust
 *
 * [203] 移除链表元素
 */

// @lc code=start

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummyHead = Box::new(ListNode::new(0));
        dummyHead.next = head;
        let mut cur = dummyHead.as_mut();
        while let Some(node) = cur.next.take() {
            if node.val == val {
                cur.next = node.next;
            } else {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummyHead.next
    }
}
// @lc code=end
// mod test {
//     use super::Solution;

//     #[test]
//     fn it_works() {
//         let n = vec![2,3,1,2,4,3];
//         let t = 7;
//         let size = Solution::min_sub_array_len(t, n);
//         // println!("{:?}", n);
//         println!("{}", size);
//     }
// }