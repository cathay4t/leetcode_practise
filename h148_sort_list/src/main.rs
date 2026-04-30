// SPDX-License-Identifier: Apache-2.0

trait CanBeNode: PartialEq + Ord + std::fmt::Display {}

impl<T> CanBeNode for T where T: PartialEq + Ord + std::fmt::Display {}

struct ListNode<T>
where
    T: CanBeNode,
{
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T>
where
    T: CanBeNode,
{
    fn sort(head: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
        // no need to sort empty or single node
        if head.is_none() || head.as_ref()?.next.is_none() {
            return head;
        }

        let right_half = Self::split_middle(&head);
        let left_sorted = Self::sort(head);
        let right_sorted = Self::sort(right_half);

        Self::merge_sort(left_sorted, right_sorted)
    }

    fn split_middle(
        head: &Option<Box<ListNode<T>>>,
    ) -> Option<Box<ListNode<T>>> {
        let mut fast = head;
        let mut slow = head;

        while fast.as_ref().and_then(|f| f.next.as_ref()).is_some() {
            fast = &fast.as_ref()?.next.as_ref()?.next;
            slow = &slow.as_ref()?.next;
        }
        // need to break as slow, need to change slow from as_ref() to as_mut()
        #[allow(mutable_transmutes)]
        let slow: &mut Option<Box<ListNode<T>>> =
            unsafe { std::mem::transmute(slow) };
        slow.take()
    }

    fn merge_sort(
        mut left: Option<Box<ListNode<T>>>,
        mut right: Option<Box<ListNode<T>>>,
    ) -> Option<Box<ListNode<T>>> {
        // we cannot generate dummy head because there is no T::empty()
        // hence we need to find the first node.
        let mut head = match (left.take(), right.take()) {
            (Some(mut l), Some(mut r)) => {
                // take the smallest as head
                if l.val < r.val {
                    left = l.next.take();
                    right = Some(r);
                    l
                } else {
                    right = r.next.take();
                    left = Some(l);
                    r
                }
            }
            (Some(v), None) | (None, Some(v)) => {
                return Some(v);
            }
            (None, None) => {
                return None;
            }
        };
        let mut tail = &mut head;

        while let (Some(l), Some(r)) = (&left, &right) {
            if l.val < r.val {
                tail.next = left.take();
                tail = tail.next.as_mut()?;
                left = tail.next.take();
            } else {
                tail.next = right.take();
                tail = tail.next.as_mut()?;
                right = tail.next.take();
            }
        }
        // at least one half if empty
        tail.next = left.or(right);

        Some(head)
    }

    fn push(&mut self, val: T) {
        if let Some(next) = self.next.as_mut() {
            next.push(val);
        } else {
            self.next = Some(Box::new(ListNode { val, next: None }));
        }
    }

    fn new(val: T) -> Self {
        Self { val, next: None }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        ListNodeIter { cur: Some(self) }
    }
}

struct ListNodeIter<'a, T>
where
    T: CanBeNode,
{
    cur: Option<&'a ListNode<T>>,
}

impl<'a, T> Iterator for ListNodeIter<'a, T>
where
    T: CanBeNode,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur.take()?;
        self.cur = cur.next.as_deref();
        Some(&cur.val)
    }
}

fn main() {
    let mut list = ListNode::new(100i32);
    list.push(98);
    list.push(94);
    list.push(93);
    list.push(100);
    println!("Before sort {:?}", list.iter().collect::<Vec<&i32>>());
    let list = ListNode::sort(Some(Box::new(list)));
    println!(
        "After sort  {:?}",
        list.as_ref().unwrap().iter().collect::<Vec<&i32>>()
    );

    assert_eq!(
        list.unwrap().iter().map(|v| *v).collect::<Vec<i32>>(),
        vec![93, 94i32, 98, 100, 100]
    );

    println!("Pass!");
}
