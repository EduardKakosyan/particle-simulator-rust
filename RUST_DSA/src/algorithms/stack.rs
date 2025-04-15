#[derive(Debug)]
pub struct Stack<T> {
    size: usize,  // stack size
    data: Vec<T>, // stack data
}

impl<T> Stack<T> {
    // initialize the stack
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    // check if the stack is empty
    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    // clear the stack
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    // push
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    // pop
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.data.get(self.size - 1)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        self.data.get_mut(self.size - 1)
    }

    /// Stack.into_iter() stack is modified and becomes a iterator
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    /// iter: stack unmodofied and get a unmutable iterator
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }

        iterator
    }
    /// iter_mut: stack unmodified and get a mutable iterator
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }

        iterator
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
