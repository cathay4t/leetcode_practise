// SPDX-License-Identifier: Apache-2.0

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut ret = vec![self.val];
        let mut cur = self.next.as_ref();
        while let Some(c) = cur.as_ref() {
            ret.push(c.val);
            cur = c.next.as_ref();
        }

        ret
    }
}

fn main() {
    let mut list_a = ListNode::new(1);
    let mut node_a1 = ListNode::new(2);
    let mut node_a2 = ListNode::new(3);
    let mut node_a3 = ListNode::new(4);
    let node_a4 = ListNode::new(5);

    node_a3.next = Some(Box::new(node_a4));
    node_a2.next = Some(Box::new(node_a3));
    node_a1.next = Some(Box::new(node_a2));
    list_a.next = Some(Box::new(node_a1));

    let result = Solution::swap_pairs(Some(Box::new(list_a))).unwrap();

    assert_eq!(result.to_vec(), vec![2, 1, 4, 3, 5]);

    let list_b = ListNode::new(1);

    let result = Solution::swap_pairs(Some(Box::new(list_b))).unwrap();

    assert_eq!(result.to_vec(), vec![1]);

    println!("PASS");
}

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;

        if head.as_ref()?.next.is_none() {
            return head;
        }
        if let Some(mut right) = head.as_mut()?.next.take()
            && let Some(mut left) = head.take()
        {
            left.next = Self::swap_pairs(right.next.take());
            right.next = Some(left);
            head = Some(right);
        }

        head
    }
}
