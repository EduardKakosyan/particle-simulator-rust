//! Deque Data structure
/// ALllows items to be added or removed from both ends. This flexibility makes it a hybrid linear
/// structure.
/// Includes:
/// new() - creates new deque stacked with no arguments and returns an empty deque
/// add_front(item) - adds an item to the front
/// add_rear(item) - adds an item to the rear
/// remove_front(item) - removes an item from the front
/// remove_rear(item) - removes an item from the rear
/// is_empty() - checks if it is empty
/// size() - returns the lengths of the deque
/// iter() - iterator (immutable)
/// iter_mut() - iterator (mutable)
/// into_iter() - changes the deque into an iterable from
#[allow(dead_code)]
#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(_cap: usize) -> Self {
        Self {
            cap: _cap,
            data: Vec::with_capacity(_cap),
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == self.len()
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.push(val);
        Ok(())
    }

    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if self.len() > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }

        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }

        iterator
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new(1)
    }
}

pub struct IntoIter<T>(Deque<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.data.remove(0))
        } else {
            None
        }
    }
}

pub struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() {
            Some(self.stack.remove(0))
        } else {
            None
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() {
            Some(self.stack.remove(0))
        } else {
            None
        }
    }
}

impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
