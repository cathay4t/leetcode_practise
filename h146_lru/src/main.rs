// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default, PartialEq, Clone)]
struct Entry {
    key: i32,
    value: i32,
    // The Rc and RefCell is not thread safe(not Sync)
    prev: Option<Rc<RefCell<Entry>>>,
    next: Option<Rc<RefCell<Entry>>>,
}

impl std::fmt::Display for Entry {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{} ", self.key, self.value)?;
        if let Some(prev) = self.prev.as_ref() {
            write!(f, "prev: {} ", prev.borrow().key)?;
        } else {
            write!(f, "prev: n ")?;
        }
        if let Some(next) = self.next.as_ref() {
            write!(f, "next: {}", next.borrow().key)?;
        } else {
            write!(f, "next: n")?;
        }
        Ok(())
    }
}

struct LRUCache {
    // HashMap<key, (value, index)>
    data: HashMap<i32, Rc<RefCell<Entry>>>,
    order_head: Option<Rc<RefCell<Entry>>>,
    order_tail: Option<Rc<RefCell<Entry>>>,
    capacity: usize,
}

impl std::fmt::Display for LRUCache {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        let mut cur = self.order_head.clone();
        while let Some(now) = cur {
            write!(f, "\n\t{}", now.borrow())?;
            if Some(now.borrow().key)
                == now.borrow().next.as_ref().map(|n| n.borrow().key)
            {
                panic!("Dead loop on key: {}", now.borrow().key);
            }
            cur = now.borrow().next.clone();
        }
        if let Some(head) = self.order_head.as_ref() {
            write!(f, "\nhead: {} ", head.borrow().key)?;
        } else {
            write!(f, "\nhead: n ")?;
        }
        if let Some(tail) = self.order_tail.as_ref() {
            write!(f, "tail: {}", tail.borrow().key)?;
        } else {
            write!(f, "tail: n")?;
        }

        Ok(())
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
            order_head: None,
            order_tail: None,
            capacity,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // TODO: To improve performance, reuse existing
        self.remove(&key);
        if self.data.len() == self.capacity {
            self.remove_tail()
        }
        self.push(key, value);
    }

    fn push(&mut self, key: i32, value: i32) {
        if self.data.get(&key).is_some() {
            panic!("Duplicate key {key} found, should remove first");
        }
        let entry = Rc::new(RefCell::new(Entry {
            key,
            value,
            prev: None,
            next: self.order_head.clone(),
        }));
        if let Some(old_head) = self.order_head.take() {
            old_head.borrow_mut().prev = Some(entry.clone());
        }
        self.order_head = Some(entry.clone());
        if self.order_tail.is_none() {
            self.order_tail = Some(entry.clone());
        }
        if self.data.len() == self.capacity {
            self.remove_tail()
        }
        self.data.insert(key, entry);
    }

    fn remove_tail(&mut self) {
        if let Some(tail) = self.order_tail.take() {
            // TODO: To improve performance, reuse existing instead of
            // search again
            self.remove(&tail.borrow().key);
        }
    }

    fn remove(&mut self, key: &i32) -> Option<i32> {
        if let Some(entry) = self.data.remove(key) {
            if entry.borrow().prev.is_none() {
                // entry is head
                self.order_head = entry.borrow().next.clone();
            }
            if entry.borrow().next.is_none() {
                // entry is tail
                self.order_tail = entry.borrow().prev.clone();
            }
            if let Some(prev) = entry.borrow().prev.as_ref() {
                prev.borrow_mut().next = entry.borrow().next.clone();
            }
            if let Some(next) = entry.borrow().next.as_ref() {
                next.borrow_mut().prev = entry.borrow().prev.clone();
            }
            Some(entry.borrow().value)
        } else {
            None
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(val) = self.remove(&key) {
            self.push(key, val);
            val
        } else {
            -1
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main() {
    let expected = vec![
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        -1,
        i32::MAX,
        19,
        17,
        i32::MAX,
        -1,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        -1,
        i32::MAX,
        -1,
        5,
        -1,
        12,
        i32::MAX,
        i32::MAX,
        3,
        5,
        5,
        i32::MAX,
        i32::MAX,
        1,
        i32::MAX,
        -1,
        i32::MAX,
        30,
        5,
        30,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        -1,
        i32::MAX,
        -1,
        24,
        i32::MAX,
        i32::MAX,
        18,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        -1,
        i32::MAX,
        i32::MAX,
        18,
        i32::MAX,
        i32::MAX,
        -1,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        18,
        i32::MAX,
        i32::MAX,
        -1,
        i32::MAX,
        4,
        29,
        30,
        i32::MAX,
        12,
        -1,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        29,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        17,
        22,
        18,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        -1,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        20,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        -1,
        18,
        18,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        20,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
        i32::MAX,
    ];

    let mut lru_cache = LRUCache::new(10);
    for (index, v) in vec![
        vec![10i32, 13],
        vec![3, 17],
        vec![6, 11],
        vec![10, 5],
        vec![9, 10],
        vec![13],
        vec![2, 19],
        vec![2],
        vec![3],
        vec![5, 25],
        vec![8],
        vec![9, 22],
        vec![5, 5],
        vec![1, 30],
        vec![11],
        vec![9, 12],
        vec![7],
        vec![5],
        vec![8],
        vec![9],
        vec![4, 30],
        vec![9, 3],
        vec![9],
        vec![10],
        vec![10],
        vec![6, 14],
        vec![3, 1],
        vec![3],
        vec![10, 11],
        vec![8],
        vec![2, 14],
        vec![1],
        vec![5],
        vec![4],
        vec![11, 4],
        vec![12, 24],
        vec![5, 18],
        vec![13],
        vec![7, 23],
        vec![8],
        vec![12],
        vec![3, 27],
        vec![2, 12],
        vec![5],
        vec![2, 9],
        vec![13, 4],
        vec![8, 18],
        vec![1, 7],
        vec![6],
        vec![9, 29],
        vec![8, 21],
        vec![5],
        vec![6, 30],
        vec![1, 12],
        vec![10],
        vec![4, 15],
        vec![7, 22],
        vec![11, 26],
        vec![8, 17],
        vec![9, 29],
        vec![5],
        vec![3, 4],
        vec![11, 30],
        vec![12],
        vec![4, 29],
        vec![3],
        vec![9],
        vec![6],
        vec![3, 4],
        vec![1],
        vec![10],
        vec![3, 29],
        vec![10, 28],
        vec![1, 20],
        vec![11, 13],
        vec![3],
        vec![3, 12],
        vec![3, 8],
        vec![10, 9],
        vec![3, 26],
        vec![8],
        vec![7],
        vec![5],
        vec![13, 17],
        vec![2, 27],
        vec![11, 15],
        vec![12],
        vec![9, 19],
        vec![2, 15],
        vec![3, 16],
        vec![1],
        vec![12, 17],
        vec![9, 1],
        vec![6, 19],
        vec![4],
        vec![5],
        vec![5],
        vec![8, 1],
        vec![11, 7],
        vec![5, 2],
        vec![9, 28],
        vec![1],
        vec![2, 2],
        vec![7, 4],
        vec![4, 22],
        vec![7, 24],
        vec![9, 26],
        vec![13, 28],
        vec![11, 26],
    ]
    .into_iter()
    .enumerate()
    {
        if v.len() == 1 {
            eprintln!("HAHA560 get {}: {}", v[0], lru_cache);
            assert_eq!(lru_cache.get(v[0]), expected[index]);
        } else {
            lru_cache.put(v[0], v[1]);
            eprintln!("HAHA560 put {}: {}", v[0], lru_cache);
        }
    }

    eprintln!("PASS");
}
