//! # Linked List Abstract Data Type
//!
//! This module defines a singly linked list as an abstract data type (ADT).
//! A linked list is an ordered collection of nodes, where each node contains
//! an element and a reference to the next node in the sequence. The list is
//! traversable from the head node.
//!
//! ## Structure
//! Each `Node<T>` in the list contains:
//! - An element of generic type `T`
//! - A reference (pointer) to the next node
//!
//! The linked list maintains a pointer to the head node. All operations start
//! from this head pointer.
//!
//! ## Operations
//!
//! - `new()`: Creates a new empty linked list. Returns a new instance with a `None` head.
//! - `push(item)`: Adds a new node containing `item` to the front (head) of the list.
//! - `pop()`: Removes and returns the current head node from the list.
//! - `peek()`: Returns an immutable reference to the head node (if any).
//! - `peek_mut()`: Returns a mutable reference to the head node (if any).
//! - `is_empty()`: Returns `true` if the list is empty, `false` otherwise.
//! - `size()`: Returns the number of elements in the list as a `usize`.
//! - `iter()`: Returns an iterator over immutable references to the list's elements.
//! - `iter_mut()`: Returns an iterator over mutable references to the list's elements.
//! - `into_iter()`: Consumes the list and returns an iterator that takes ownership of each element.
//!
//! ## Example Use
//! ```rust
//! let mut list = LinkedList::new();
//! list.push(1);
//! list.push(2);
//! assert_eq!(list.pop().unwrap(), 2);
//! assert_eq!(list.size(), 1);
//! ```
//!
//! `Link<num>` in examples refers to a pointer to the node containing `num`, with
//! the left-most node always being the current head.
/// In rust `Box<T>` is a smart pointer that allocated to the heap
/// instead of the stack this is useful when you don't want to place
/// huge objects with unknown size (dynamic) on the stack to avoid
/// overflow.
type Link<T> = Option<Box<Node<T>>>;
pub struct List<T> {
    size: usize,
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IntoIter<T>(List<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}
