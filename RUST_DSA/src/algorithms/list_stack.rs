#![allow(dead_code)]
#[derive(Debug, Clone)]

struct Node<T> {
    data: T,
    next: Link<T>,
}

// Node Self-contained Reference
type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

#[derive(Debug, Clone)]
struct LStack<T> {
    size: usize,
    top: Link<T>,
}

impl<T: Clone> LStack<T> {
    fn new() -> Self {
        Self { size: 0, top: None }
    }
}
