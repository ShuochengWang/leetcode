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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut list = head.take();

        let mut cur = &list;
        let mut len = 0;
        while let Some(node) = cur {
            cur = &node.next;
            len += 1;
        }

        let mut cur = &mut list;
        for _ in 0..(len - 1) / 2 {
            cur = &mut cur.as_mut().unwrap().next;
        }

        let mut right_list = cur.as_mut().unwrap().next.take();
        let mut right_list = Self::reverse(right_list);
        let mut left_list = list;

        let mut cur = &mut left_list;
        while let Some(mut node) = right_list {
            right_list = node.next;
            node.next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = Some(node);
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        *head = left_list;
    }

    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}