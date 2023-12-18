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
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;
        let mut heap = BinaryHeap::new();
        for list in lists {
            if let Some(node) = list {
                heap.push(Reverse(node));
            }
        }
        
        while let Some(Reverse(mut node)) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(Reverse(next));
            }
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }

        dummy.next
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
