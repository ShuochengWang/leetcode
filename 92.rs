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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = &mut dummy;
        for _ in 1..left {
            prev = &mut prev.as_mut()?.next;
        }

        let mut node1 = prev.as_mut()?.next.take();
        let mut node2 = node1.as_mut()?.next.take();
        for _ in left..right {
            let node3 = node2.as_mut()?.next.take();
            node2.as_mut()?.next = node1;
            node1 = node2;
            node2 = node3;
        }

        let mut after = node2;
        let mut sub_head = node1;
        let mut sub_tail = &mut sub_head;
        for _ in left..right {
            sub_tail = &mut sub_tail.as_mut()?.next;
        }

        sub_tail.as_mut()?.next = after;
        prev.as_mut()?.next = sub_head;
        dummy?.next
    }
}