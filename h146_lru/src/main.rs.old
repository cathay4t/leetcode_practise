// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default, PartialEq, Clone)]
struct Entry {
    key: i32,
    data: i32,
    // The Rc and RefCell is not thread safe(not Sync)
    prev: Option<Rc<RefCell<Entry>>>,
    next: Option<Rc<RefCell<Entry>>>,
}

impl Entry {
    fn remove(&mut self) {
        let prev = self.prev.take();
        let next = self.next.take();
        if let Some(p) = prev.clone() {
            p.borrow_mut().next = next.clone();
        }
        if let Some(n) = next.clone() {
            n.borrow_mut().prev = prev.clone();
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{} ", self.key, self.data)?;
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
    fn new(capacity: i32) -> Self {
        let capacity: usize = if capacity < 0 {
            // Reject everything
            0usize
        } else {
            capacity as usize
        };
        Self {
            data: HashMap::with_capacity(capacity),
            order_head: None,
            order_tail: None,
            capacity,
        }
    }

    fn append_to_head(&mut self, entry: Rc<RefCell<Entry>>) {
        if let Some(order_head) = self.order_head.as_mut() {
            (*order_head.borrow_mut()).prev = Some(entry.clone());
            (*entry.borrow_mut()).next = Some(order_head.clone());
            (*entry.borrow_mut()).prev = None;
            *order_head = entry.clone();
        } else {
            self.order_head = Some(entry.clone());
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        let Some(entry) = self.data.get(&key).cloned() else {
            return -1;
        };
        if self.capacity == 1 {
            // no change required for capacity 1
            entry.borrow().data
        } else {
            if entry.borrow().prev.is_none() {
                // entry is already head, do nothing
                return entry.borrow().data;
            }

            // if entry is tail, update tail
            if entry.borrow().next.is_none() {
                self.order_tail = entry.borrow().prev.clone();
                if let Some(order_tail) = self.order_tail.as_ref() {
                    (*order_tail.borrow_mut()).next = None;
                }
            }
            // Remove it from chain
            entry.borrow_mut().remove();
            // Append to head
            self.append_to_head(entry.clone());
            entry.borrow().data
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        let entry = Rc::new(RefCell::new(Entry {
            key: key,
            data: value,
            prev: None,
            // We are not sure next yet, if we are replacing head, we should
            // point to old data, hence set to None for now
            next: None,
        }));
        if self.capacity == 1 {
            self.data = HashMap::new();
            self.data.insert(key, entry.clone());
            self.order_head = Some(entry.clone());
            self.order_tail = Some(entry);
            return;
        }
        if let Some(old_entry) = self.data.insert(key, entry.clone()) {
            // overriding
            if old_entry.borrow().prev.is_none() {
                // old data is head
                (*entry.borrow_mut()).next = old_entry.borrow().next.clone();
                if let Some(old_entry_next) = old_entry.borrow().next.as_ref() {
                    (*old_entry_next.borrow_mut()).prev = Some(entry.clone());
                }
                self.order_head = None;
                // old data is also tail
                if old_entry.borrow().next.is_none() {
                    // old data is also head
                    self.order_tail = Some(entry.clone());
                }
            } else if old_entry.borrow().next.is_none() {
                // old data is tail but not head
                self.order_tail = old_entry.borrow().prev.clone();
                old_entry.borrow_mut().remove();
            } else {
                // old data is in middle
                old_entry.borrow_mut().remove();
            }
            self.append_to_head(entry.clone());
        } else {
            self.append_to_head(entry.clone());
            if self.order_tail.is_none() {
                self.order_tail = Some(entry.clone());
                return;
            }
        }

        // Remove if exceeded capacity
        if self.data.len() > self.capacity {
            if let Some(tail) = self.order_tail.take() {
                self.data.remove(&tail.borrow().key);
                self.order_tail = tail.borrow().prev.clone();
                tail.borrow_mut().remove();
            } else {
                panic!(
                    "BUG: Exceeded capacity but got empty tail: {:?}",
                    self.data.keys()
                );
            }
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
    let mut lru_cache = LRUCache::new(2);
    lru_cache.put(2, 1);
    lru_cache.put(2, 2);
    assert_eq!(lru_cache.get(2), 2);
    lru_cache.put(3, 3);
    assert_eq!(lru_cache.get(2), 2);
    lru_cache.put(1, 1);
    lru_cache.put(4, 1);
    assert_eq!(lru_cache.get(2), -1);


    let mut lru_cache = LRUCache::new(1);
    lru_cache.put(2, 1);
    assert_eq!(lru_cache.get(2), 1);
    lru_cache.put(3, 2);
    assert_eq!(lru_cache.get(2), -1);
    assert_eq!(lru_cache.get(3), 2);

    let mut lru_cache = LRUCache::new(2);
    lru_cache.put(2, 1);
    lru_cache.put(2, 2);
    assert_eq!(lru_cache.get(2), 2);
    lru_cache.put(1, 1);
    lru_cache.put(4, 1);
    assert_eq!(lru_cache.get(2), -1);

    let mut lru_cache = LRUCache::new(2);
    lru_cache.put(2, 1);
    lru_cache.put(1, 1);
    lru_cache.put(2, 3);
    lru_cache.put(4, 1);
    assert_eq!(lru_cache.get(1), -1);
    assert_eq!(lru_cache.get(2), 3);

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
