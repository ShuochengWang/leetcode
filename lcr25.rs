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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = Self::reverse(l1);
        let mut l2 = Self::reverse(l2);
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;
        let mut carry = false;
        while l1.is_some() || l2.is_some() {
            let mut res = 0;
            if let Some(node) = l1 {
                res += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                res += node.val;
                l2 = node.next;
            }
            if carry {
                res += 1;
                carry = false;
            }
            if res > 9 {
                carry = true;
                res = res % 10;
            }
            cur.next = Some(Box::new(ListNode::new(res)));
            cur = cur.next.as_mut()?;
        }
        if carry {
            cur.next = Some(Box::new(ListNode::new(1)));
        }
        Self::reverse(dummy.next.take())
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