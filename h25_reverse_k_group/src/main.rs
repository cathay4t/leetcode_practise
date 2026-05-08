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
    let mut node_a4 = ListNode::new(5);
    let mut node_a5 = ListNode::new(6);
    let node_a6 = ListNode::new(7);

    node_a5.next = Some(Box::new(node_a6));
    node_a4.next = Some(Box::new(node_a5));
    node_a3.next = Some(Box::new(node_a4));
    node_a2.next = Some(Box::new(node_a3));
    node_a1.next = Some(Box::new(node_a2));
    list_a.next = Some(Box::new(node_a1));

    let result = Solution::reverse_k_group(Some(Box::new(list_a)), 3).unwrap();

    assert_eq!(result.to_vec(), vec![3, 2, 1, 6, 5, 4, 7]);

    let list_b = ListNode::new(1);

    let result = Solution::reverse_k_group(Some(Box::new(list_b)), 1).unwrap();

    assert_eq!(result.to_vec(), vec![1]);

    println!("PASS");
}

struct Solution;

impl Solution {
    pub fn reverse_k_group(
        head: Option<Box<ListNode>>,
        k: i32,
    ) -> Option<Box<ListNode>> {
        if head.as_ref()?.next.is_none() {
            return head;
        }
        let mut head = head;
        let mut next_group_head = &mut head;
        // find the next_group head;
        for _ in 0..k {
            if let Some(n) = next_group_head {
                next_group_head = &mut n.next;
            } else {
                return head;
            }
        }
        // request next group to reverse and return next group new head.
        let mut next_node = Self::reverse_k_group(next_group_head.take(), k);

        let mut cur = head;
        for _ in 0..k {
            // first node should next to next group head.
            // second node should next to first node.
            // etc
            if let Some(mut n) = cur.take() {
                cur = n.next.take();
                n.next = next_node.take();
                next_node = Some(n);
            }
        }

        next_node
    }
}
