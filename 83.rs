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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut head = head;
        let mut cur = head.as_mut()?;
        while let Some(next) = cur.next.as_mut() {
            if cur.val == next.val {
                cur.next = next.next.take();
            } else {
                cur = cur.next.as_mut()?;
            }
        }
        head
    }
}