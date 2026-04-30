// SPDX-License-Identifier: Apache-2.0

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn push(&mut self, val: i32) {
        let mut cur = &mut self.next;
        while let Some(node) = cur {
            cur = &mut node.next;
        }
        *cur = Some(Box::new(Self::new(val)));
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut ret = vec![self.val];
        let mut cur = self.next.as_ref();
        while let Some(c) = cur {
            ret.push(c.val);
            cur = c.next.as_ref();
        }
        ret
    }
}

impl Solution {
    /// 1. Find the middle
    /// 2. Reverse the order of right-half
    /// 3. Check two halves one-by-one
    /// 4. Optional: Restore the right-half
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let right_half = Self::reverse(Self::find_the_middle(&head));

        let mut right = &right_half;
        let mut left = &head;

        while let (Some(l), Some(r)) = (left, right) {
            if l.val != r.val {
                return false;
            }
            left = &l.next;
            right = &r.next;
        }

        true
    }

    fn find_the_middle(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head;
        let mut slow: &Option<Box<ListNode>> = head;
        // 0, 1, 2, 3, 4
        // ^*
        //    ^  *
        //       ^     *
        // For even number of items, we take the slow pointer as tail of
        // right-half.
        // For odd number of items, we take slow pointer as tail of
        // right-half, the middle item should be ignored during comparison
        // because left-half is shorter.

        while fast.as_ref().and_then(|f| f.next.as_ref()).is_some() {
            // safe to unwrap as we already checked fast pointer
            slow = &slow.as_ref()?.next;
            // safe to unwrap as we already checked fast-next pointer
            fast = &fast.as_ref()?.next.as_ref()?.next;
        }

        #[allow(mutable_transmutes)]
        let mut slow: &mut Option<Box<ListNode>> =
            unsafe { std::mem::transmute(slow) };

        slow.take()
    }

    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut cur = head;

        while cur.is_some() {
            let mut c = cur.take()?;
            cur = c.next.take();
            c.next = prev;
            prev = Some(c);
        }
        prev
    }
}

struct Solution;

fn main() {
    let mut head = ListNode::new(1);
    head.push(2);
    head.push(2);
    head.push(1);

    assert_eq!(Solution::is_palindrome(Some(Box::new(head))), true);

    println!("PASS");
}
