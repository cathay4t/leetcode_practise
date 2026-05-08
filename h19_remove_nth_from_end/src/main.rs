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

    let result =
        Solution::remove_nth_from_end(Some(Box::new(list_a)), 2).unwrap();

    assert_eq!(result.to_vec(), vec![1, 2, 3, 5]);

    let mut list_b = ListNode::new(1);
    let node_b1 = ListNode::new(2);
    list_b.next = Some(Box::new(node_b1));

    let result =
        Solution::remove_nth_from_end(Some(Box::new(list_b)), 1).unwrap();

    assert_eq!(result.to_vec(), vec![1]);

    println!("PASS");
}

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        if n <= 0 {
            return head;
        }
        // return None if empty
        head.as_ref()?;

        let mut cur = head.as_ref();

        for _ in 0..n {
            if let Some(c) = cur.as_ref() {
                cur = c.next.as_ref();
            }
        }
        let mut nth_pre: Option<&Box<ListNode>> = None;
        let mut nth = head.as_ref();

        while let Some(c) = cur.as_ref() {
            cur = c.next.as_ref();
            nth_pre = if let Some(p) = nth_pre {
                p.next.as_ref()
            } else {
                head.as_ref()
            };
            nth = nth.as_ref().and_then(|n| n.next.as_ref());
        }

        if let Some(n) = nth {
            if n == head.as_ref()? {
                println!("HAHA630 removing head");
                head?.next.take()
            } else {
                #[allow(mutable_transmutes)]
                let n: &mut Box<ListNode> = unsafe { std::mem::transmute(n) };

                if let Some(pre) = nth_pre {
                    #[allow(mutable_transmutes)]
                    let pre: &mut Box<ListNode> =
                        unsafe { std::mem::transmute(pre) };
                    pre.next = n.next.take();
                    head
                } else {
                    // removing head
                    panic!("HAHA947 nth_pre is None but nth is not head");
                }
            }
        } else {
            head
        }
    }
}
