// SPDX-License-Identifier: Apache-2.0

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
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
    let mut lista = ListNode::new(1);
    let mut lista1 = ListNode::new(2);
    let lista2 = ListNode::new(4);
    lista1.next = Some(Box::new(lista2));
    lista.next = Some(Box::new(lista1));

    let mut listb = ListNode::new(1);
    let mut listb1 = ListNode::new(3);
    let listb2 = ListNode::new(4);
    listb1.next = Some(Box::new(listb2));
    listb.next = Some(Box::new(listb1));

    let merged =
        Solution::merge_two_lists(Some(Box::new(lista)), Some(Box::new(listb)));

    assert_eq!(merged.unwrap().to_vec(), vec![1, 1, 2, 3, 4, 4]);

    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ret = ListNode::new(i32::MAX);
        let mut tail = &mut ret;

        let mut node_a = list1;
        let mut node_b = list2;

        loop {
            match (node_a.take(), node_b.take()) {
                (None, None) => {
                    break;
                }
                (Some(a), None) => {
                    tail.next = Some(a);
                    break;
                }
                (None, Some(b)) => {
                    tail.next = Some(b);
                    break;
                }
                (Some(mut a), Some(mut b)) => {
                    if a.val < b.val {
                        node_a = a.next.take();
                        node_b = Some(b);
                        tail.next = Some(a);
                        tail = tail.next.as_mut()?;
                    } else if a.val > b.val {
                        node_a = Some(a);
                        node_b = b.next.take();
                        tail.next = Some(b);
                        tail = tail.next.as_mut()?;
                    } else {
                        node_a = a.next.take();
                        node_b = b.next.take();
                        a.next = Some(b);
                        tail.next = Some(a);
                        tail = tail.next.as_mut()?.next.as_mut()?;
                    }
                }
            }
        }

        ret.next.take()
    }
}
