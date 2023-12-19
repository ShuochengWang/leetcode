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
    fn detect_cycle(head: Option<Box<ListNode>>) -> i32 {
        let mut fast = &head;
        let mut slow = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
            if fast == slow {
                break;
            }
        }

        if fast.is_none() || fast.as_ref().unwrap().next.is_none() {
            return -1;
        }

        slow = &head;
        let mut res = 0;
        while slow != fast {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
            res += 1;
        }
        res
    }
}