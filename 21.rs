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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut list1;
        while list2.is_some() {
            if cur.is_none() || cur.as_ref().unwrap().val > list2.as_ref().unwrap().val {
                std::mem::swap(cur, &mut list2);
            }
            cur = &mut cur.as_mut().unwrap().next;
        }

        list1
    }

    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                cur.next = list1.take();
                cur = cur.next.as_mut().unwrap();
                list1 = cur.next.take();
            } else {
                cur.next = list2.take();
                cur = cur.next.as_mut().unwrap();
                list2 = cur.next.take();
            }
        }

        if list1.is_some() {
            cur.next = list1;
        }
        if list2.is_some() {
            cur.next = list2;
        }

        dummy.next
    }
}