// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;

trait CanBeNode: std::fmt::Display + PartialEq + PartialOrd + Ord {}

impl<T> CanBeNode for T where T: std::fmt::Display + PartialEq + PartialOrd + Ord
{}

struct BinarySearchTree<T>
where
    T: CanBeNode,
{
    root: Option<Box<BstNode<T>>>,
}

impl<T> std::fmt::Display for BinarySearchTree<T>
where
    T: CanBeNode,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "\nTree:")?;
        for node in self.iter() {
            write!(fmt, "\n\t{}", node)?;
        }
        Ok(())
    }
}

impl<T> std::fmt::Debug for BinarySearchTree<T>
where
    T: CanBeNode,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = self.root.as_ref() {
            write!(fmt, "{:?}", root)?;
        } else {
            write!(fmt, "empty")?;
        }
        Ok(())
    }
}

impl<T> BinarySearchTree<T>
where
    T: CanBeNode,
{
    fn new(value: T) -> Self {
        Self {
            root: Some(Box::new(BstNode::new(value))),
        }
    }

    //TODO: return &T instead of RefCell
    fn iter(&self) -> impl Iterator<Item = &T> {
        let mut stack = Vec::new();

        let mut current = self.root.as_deref();
        while let Some(c) = current {
            stack.push(c);
            current = c.left.as_deref();
        }
        BstIter { stack }
    }

    fn insert(&mut self, value: T) {
        log::debug!("Insert {value} to {self}");
        if let Some(root) = self.root.as_mut() {
            root.insert(value);
        } else {
            *self = Self::new(value);
        }
    }

    fn remove(&mut self, value: T) {
        log::debug!("Remove {value} from {self}");
        if let Some(root) = self.root.take() {
            self.root = root.remove(value);
        }
    }
}

/// Binary Search Tree Node
struct BstNode<T>
where
    T: CanBeNode,
{
    value: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>,
}

impl<T> BstNode<T>
where
    T: CanBeNode,
{
    fn insert(&mut self, value: T) {
        if value < self.value {
            if let Some(left) = self.left.as_mut() {
                left.insert(value);
            } else {
                self.left = Some(Box::new(BstNode::new(value)));
            }
        } else if value > self.value {
            if let Some(right) = self.right.as_mut() {
                right.insert(value);
            } else {
                self.right = Some(Box::new(BstNode::new(value)));
            }
        } else {
            log::info!("Ignore duplicate key");
        }
    }

    // Remove the minimum child from tree
    // Return the child value and new tree
    fn remove_min_child(mut self: Box<Self>) -> (T, Option<Box<Self>>) {
        if let Some(left) = self.left.take() {
            let (val, new_left) = left.remove_min_child();
            self.left = new_left;
            (val, Some(self))
        } else {
            (self.value, self.right)
        }
    }

    /// Return the new tree root
    fn remove(mut self: Box<Self>, value: T) -> Option<Box<Self>> {
        if value < self.value {
            if let Some(left) = self.left.take() {
                self.left = left.remove(value);
            }
            Some(self)
        } else if value > self.value {
            if let Some(right) = self.right.take() {
                self.right = right.remove(value);
            }
            Some(self)
        } else {
            match (self.left.take(), self.right.take()) {
                (None, None) => None,
                (Some(v), None) | (None, Some(v)) => Some(v),
                (Some(left), Some(right)) => {
                    let (value, new_right) = right.remove_min_child();
                    Some(Box::new(Self {
                        value,
                        left: Some(left),
                        right: new_right,
                    }))
                }
            }
        }
    }
}

impl<T> std::fmt::Display for BstNode<T>
where
    T: CanBeNode,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "\tval {}\n", self.value)?;
        if let Some(l) = self.left.as_ref() {
            write!(fmt, "\t\tlft {}\n", l.value)?;
        } else {
            write!(fmt, "\t\tlft null\n")?;
        }
        if let Some(r) = self.right.as_ref() {
            write!(fmt, "\t\trht {}\n", r.value)?;
        } else {
            write!(fmt, "\t\trgt null\n")?;
        }
        Ok(())
    }
}

impl<T> BstNode<T>
where
    T: CanBeNode,
{
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> std::fmt::Debug for BstNode<T>
where
    T: CanBeNode,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_node<T>(
            node: &BstNode<T>,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result
        where
            T: CanBeNode,
        {
            write!(f, "{}", node.value)?;
            if node.left.is_some() || node.right.is_some() {
                write!(f, " -> (")?;
                match &node.left {
                    Some(left) => write_node(left, f)?,
                    None => write!(f, "_")?,
                }
                write!(f, ", ")?;
                match &node.right {
                    Some(right) => write_node(right, f)?,
                    None => write!(f, "_")?,
                }
                write!(f, ")")?;
            }
            Ok(())
        }

        write_node(self, f)
    }
}

struct BstIter<'a, T>
where
    T: CanBeNode,
{
    stack: Vec<&'a BstNode<T>>,
}

impl<'a, T> Iterator for BstIter<'a, T>
where
    T: CanBeNode,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_node = self.stack.pop()?;
        let mut current = next_node.right.as_deref();
        while let Some(c) = current {
            self.stack.push(c);
            current = c.left.as_deref();
        }
        Some(&next_node.value)
    }
}

fn init_logger() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();
}

fn main() {
    init_logger();
    let mut root = BinarySearchTree::new(100u32);
    for (is_insert, value, expected) in [
        (true, 100u32, vec![100u32]),
        (true, 98, vec![98, 100]),
        (true, 99, vec![98, 99, 100]),
        (false, 99, vec![98, 100]),
        (false, 100, vec![98]),
    ] {
        if is_insert {
            root.insert(value);
        } else {
            root.remove(value);
        }
        let full: Vec<u32> = root.iter().map(|v| *v).collect();
        assert_eq!(full, expected);
        log::debug!("Post: {root}");
        log::debug!("Debug: {root:?}");
    }

    println!("PASS");
}
