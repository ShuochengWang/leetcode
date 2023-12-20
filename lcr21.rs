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

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut cur = &head;
        let mut len = 0;
        while let Some(node) = cur {
            cur = &node.next;
            len += 1;
        }

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut cur = &mut dummy;
        for _ in 0..len - n as usize {
            cur = &mut cur.as_mut()?.next;
        }
        cur.as_mut()?.next = cur.as_mut()?.next.as_mut()?.next.take();
        dummy?.next
    }
}