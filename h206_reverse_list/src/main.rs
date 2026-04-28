// SPDX-License-Identifier: Apache-2.0

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct MyList {
    head: Option<Box<ListNode>>,
}

impl MyList {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, value: i32) {
        let mut tail = &mut self.head;

        while let Some(node) = tail {
            tail = &mut node.next;
        }

        *tail = Some(Box::new(ListNode::new(value)));
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut ret = Vec::new();
        let mut cur = self.head.as_ref();
        while let Some(n) = cur.as_ref() {
            ret.push(n.val);
            cur = n.next.as_ref();
        }
        ret
    }
}

fn main() {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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

    let mut list = MyList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(6);
    list.push(5);
    println!("HAHA31 {:?}", list.to_vec());

    let new_list = MyList {head: reverse_list(list.head)};
    println!("HAHA31 {:?}", new_list.to_vec());

    println!("Hello, world!");
}
