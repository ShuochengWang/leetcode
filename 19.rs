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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head}));
        let mut cur = dummy.as_ref();
        let mut len = 0;
        while let Some(node) = cur {
            cur = node.next.as_ref();
            len += 1;
        }

        let steps = len - n as usize - 1;
        let mut prev = dummy.as_mut();
        for _ in 0..steps {
            prev = prev.unwrap().next.as_mut();
        }
        prev.unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();
        dummy.unwrap().next
    }

    
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Solution::remove_nth_from_end_helper(head, n).0
    }

    fn remove_nth_from_end_helper(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
        match head {
            None => (None, 1),
            Some(mut node) => {
                let (next, num) = Solution::remove_nth_from_end_helper(node.next.take(), n);
                if num == n {
                    (next, num + 1)
                } else {
                    node.next = next;
                    (Some(node), num + 1)
                }
            }
        }
    }
}