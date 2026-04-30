// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, rc::Rc};

struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn push(&mut self, val: i32) {
        self.push_node(Rc::new(RefCell::new(ListNode { val, next: None })));
    }

    fn push_node(&mut self, node: Rc<RefCell<ListNode>>) {
        if let Some(next) = self.next.as_ref() {
            next.borrow_mut().push_node(node);
        } else {
            self.next = Some(node);
        }
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut ret = Vec::new();
        ret.push(self.val);

        let mut cur = self.next.clone();

        while let Some(c) = cur {
            ret.push(c.borrow().val);
            cur = c.borrow().next.clone();
        }

        ret
    }

    fn len(&self) -> usize {
        let mut count = 1usize;

        let mut cur = self.next.clone();
        while let Some(c) = cur {
            count += 1;
            cur = c.borrow().next.clone();
        }
        count
    }
}

fn get_intersection_node(
    list_a: Option<Rc<RefCell<ListNode>>>,
    list_b: Option<Rc<RefCell<ListNode>>>,
) -> Option<Rc<RefCell<ListNode>>> {
    // use Rc::ptr_eq() to check whether two Rc is pointing to the same
    // memory
    if list_a.is_none() || list_b.is_none() {
        return None;
    }

    let count_a = list_a
        .as_ref()
        .map(|l| l.borrow().len())
        .unwrap_or_default();
    let count_b = list_b
        .as_ref()
        .map(|l| l.borrow().len())
        .unwrap_or_default();

    let mut node_a = list_a.clone();
    let mut node_b = list_b.clone();
    // if list_a and list_b ever intersect, we should skip checking the
    // first leading difference nodes.
    if count_a > count_b {
        for i in 0..(count_a - count_b) {
            node_a = node_a.as_ref().and_then(|n| n.borrow().next.clone());
        }
    } else {
        for i in 0..(count_b - count_a) {
            node_b = node_b.as_ref().and_then(|n| n.borrow().next.clone());
        }
    };
    while let (Some(a), Some(b)) = (node_a.take(), node_b.take()) {
        if Rc::ptr_eq(&a, &b) {
            return Some(a);
        } else {
            node_a = a.borrow().next.clone();
            node_b = b.borrow().next.clone();
        }
    }

    None
}

fn main() {
    let mut list_a = ListNode { val: 4, next: None };

    list_a.push(1);

    let mut list_b = ListNode { val: 5, next: None };

    list_b.push(6);
    list_b.push(1);

    let mut list_share = ListNode { val: 8, next: None };
    for i in [4i32, 5] {
        list_share.push(i)
    }
    let list_share = Rc::new(RefCell::new(list_share));

    list_a.push_node(list_share.clone());
    list_b.push_node(list_share);

    println!("List A {:?}", list_a.to_vec());
    println!("List B {:?}", list_b.to_vec());

    assert_eq!(
        get_intersection_node(
            Some(Rc::new(RefCell::new(list_a))),
            Some(Rc::new(RefCell::new(list_b)))
        )
        .as_ref()
        .map(|v| v.borrow().val),
        Some(8)
    );

    println!("PASS");
}
