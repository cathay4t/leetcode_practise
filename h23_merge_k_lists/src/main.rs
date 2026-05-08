// SPDX-License-Identifier: Apache-2.0

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut ret = vec![self.val];

        let mut cur = self.next.as_ref();
        while let Some(c) = cur {
            ret.push(c.val);
            cur = c.next.as_ref()
        }
        ret
    }
}

struct Solution;

impl Solution {
    pub fn merge_k_lists(
        lists: Vec<Option<Box<ListNode>>>,
    ) -> Option<Box<ListNode>> {
        let mut lists = lists;
        if lists.len() < 2 {
            lists.pop()?
        } else if lists.len() == 2 {
            let mut right = lists.pop()?;
            let mut left = lists.pop()?;
            let mut dummy_head = ListNode::new(0);
            let mut tail = &mut dummy_head;

            loop {
                match (left.take(), right.take()) {
                    (Some(mut l), Some(mut r)) => {
                        if l.val < r.val {
                            left = l.next.take();
                            right = Some(r);
                            tail.next = Some(l);
                            tail = tail.next.as_mut()?;
                        } else if l.val > r.val {
                            left = Some(l);
                            right = r.next.take();
                            tail.next = Some(r);
                            tail = tail.next.as_mut()?;
                        } else {
                            left = l.next.take();
                            right = r.next.take();
                            l.next = Some(r);
                            tail.next = Some(l);
                            tail = tail.next.as_mut()?.next.as_mut()?;
                        }
                    }
                    (Some(l), None) => {
                        tail.next = Some(l);
                        break;
                    }
                    (None, Some(r)) => {
                        tail.next = Some(r);
                        break;
                    }
                    (None, None) => break,
                }
            }

            dummy_head.next.take()
        } else {
            // split array in middle
            let right = Self::merge_k_lists(lists.split_off(lists.len() / 2));
            let left = Self::merge_k_lists(lists);
            Self::merge_k_lists(vec![left, right])
        }
    }
}

fn main() {
    let mut lista = ListNode::new(93i32);
    let mut node_a1 = ListNode::new(94);
    let mut node_a2 = ListNode::new(95);
    let mut node_a3 = ListNode::new(96);
    let node_a4 = ListNode::new(100);

    node_a3.next = Some(Box::new(node_a4));
    node_a2.next = Some(Box::new(node_a3));
    node_a1.next = Some(Box::new(node_a2));
    lista.next = Some(Box::new(node_a1));

    let mut listb = ListNode::new(92i32);
    let mut node_b1 = ListNode::new(93);
    let mut node_b2 = ListNode::new(94);
    let mut node_b3 = ListNode::new(96);
    let node_b4 = ListNode::new(100);

    node_b3.next = Some(Box::new(node_b4));
    node_b2.next = Some(Box::new(node_b3));
    node_b1.next = Some(Box::new(node_b2));
    listb.next = Some(Box::new(node_b1));

    let list = Solution::merge_k_lists(vec![
        Some(Box::new(lista)),
        Some(Box::new(listb)),
    ]);

    assert_eq!(
        list.unwrap().to_vec(),
        vec![92i32, 93, 93, 94, 94, 95, 96, 96, 100, 100]
    );

    println!("Pass!");
}
