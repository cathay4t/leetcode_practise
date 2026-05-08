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
    let mut list_a = ListNode::new(2);
    let mut node_a1 = ListNode::new(4);
    let node_a2 = ListNode::new(3);

    node_a1.next = Some(Box::new(node_a2));
    list_a.next = Some(Box::new(node_a1));

    let mut list_b = ListNode::new(5);
    let mut node_b1 = ListNode::new(6);
    let node_b2 = ListNode::new(4);
    node_b1.next = Some(Box::new(node_b2));
    list_b.next = Some(Box::new(node_b1));

    let result = Solution::add_two_numbers(
        Some(Box::new(list_a)),
        Some(Box::new(list_b)),
    )
    .unwrap();

    assert_eq!(result.to_vec(), vec![7i32, 0, 8]);

    println!("PASS"); 
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ret = ListNode::new(i32::MAX);
        let mut tail = &mut ret;
        let mut ten_more = false;
        let mut l1 = l1;
        let mut l2 = l2;

        loop {
            let value = match (ten_more, l1.take(), l2.take()) {
                (false, Some(a), None) => {
                    tail.next = Some(a);
                    break;
                }
                (false, None, Some(b)) => {
                    tail.next = Some(b);
                    break;
                }
                (true, None, None) => 1,
                (false, None, None) => {
                    break;
                }
                (true, Some(mut a), None) => {
                    l1 = a.next.take();
                    a.val + 1
                }
                (true, None, Some(mut b)) => {
                    l2 = b.next.take();
                    1 + b.val
                }
                (_, Some(mut a), Some(mut b)) => {
                    l1 = a.next.take();
                    l2 = b.next.take();
                    a.val + if ten_more { 1 } else { 0 } + b.val
                }
            };

            ten_more = value >= 10;
            let value = if value >= 10 { value - 10 } else { value };

            tail.next = Some(Box::new(ListNode::new(value)));
            tail = tail.next.as_mut()?;
        }
        ret.next.take()
    }
}
