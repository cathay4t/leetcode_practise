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
    fn split_middle(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.as_ref().and_then(|f| f.next.as_ref()).is_some() {
            fast = fast.as_ref()?.next.as_ref()?.next.as_ref();
            slow = slow.as_ref()?.next.as_ref();
        }
        #[allow(mutable_transmutes)]
        let slow: &mut Option<Box<ListNode>> =
            unsafe { std::mem::transmute(slow) };
        slow.take()
    }

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref()?.next.is_none() {
            return head;
        }

        let right_half = Self::split_middle(&head);

        let mut sorted_left = Self::sort_list(head);
        let mut sorted_right = Self::sort_list(right_half);

        let mut dummy_head = ListNode::new(0);
        let mut tail = &mut dummy_head;

        loop {
            match (sorted_left.take(), sorted_right.take()) {
                (Some(mut l), Some(mut r)) => {
                    if l.val < r.val {
                        sorted_left = l.next.take();
                        sorted_right = Some(r);
                        tail.next = Some(l);
                        tail = tail.next.as_mut()?;
                    } else if l.val > r.val {
                        sorted_left = Some(l);
                        sorted_right = r.next.take();
                        tail.next = Some(r);
                        tail = tail.next.as_mut()?;
                    } else {
                        sorted_left = l.next.take();
                        sorted_right = r.next.take();
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
    }
}

fn main() {
    let mut list = ListNode::new(100i32);
    let mut node1 = ListNode::new(98);
    let mut node2 = ListNode::new(94);
    let mut node3 = ListNode::new(93);
    let node4 = ListNode::new(100);

    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));
    list.next = Some(Box::new(node1));

    println!("Before sort {:?}", list.to_vec());
    let list = Solution::sort_list(Some(Box::new(list)));

    println!("After sort  {:?}", list.as_ref().unwrap().to_vec());

    assert_eq!(list.unwrap().to_vec(), vec![93, 94i32, 98, 100, 100]);

    println!("Pass!");
}
