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
    pub fn trainning_plan(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut l1;
        while l2.is_some() {
            if cur.is_none() || cur.as_ref()?.val > l2.as_ref()?.val {
                std::mem::swap(cur, &mut l2);
            }
            cur = &mut cur.as_mut()?.next;
        }

        l1
    }
}