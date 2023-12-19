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
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            cur = &node.next;
            len += 1;
        }

        for _ in 0..len / 2 {
            head = head.unwrap().next;
        }
        head
    }
}