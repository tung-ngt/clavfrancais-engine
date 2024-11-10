use std::{collections::VecDeque, fmt::Debug};

pub trait CharBuffer : Debug {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn top(&self) -> Option<&char>;

    fn push(&mut self, element: char);

    fn pop(&mut self) -> Option<char>;

    fn clear(&mut self);
}

#[derive(Debug)]
pub struct ResizableCharBuffer {
    elements: Vec<char>,
}

impl ResizableCharBuffer {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for ResizableCharBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl CharBuffer for ResizableCharBuffer {
    fn len(&self) -> usize {
        self.elements.len()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn top(&self) -> Option<&char> {
        self.elements.last()
    }

    fn push(&mut self, element: char) {
        self.elements.push(element);
    }

    fn pop(&mut self) -> Option<char> {
        self.elements.pop()
    }

    fn clear(&mut self) {
        self.elements.clear()
    }
}

#[derive(Debug)]
pub struct FixedSizeCharBuffer {
    elements: VecDeque<char>,
    capacity: usize,
}

impl FixedSizeCharBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            elements: VecDeque::new(),
            capacity
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl CharBuffer for FixedSizeCharBuffer {
    fn len(&self) -> usize {
        self.elements.len()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn top(&self) -> Option<&char> {
        self.elements.back()
    }

    fn push(&mut self, element: char) {
        if self.len() == self.capacity() {
            self.elements.pop_front();
        }

        self.elements.push_back(element);
    }

    fn pop(&mut self) -> Option<char> {
        self.elements.pop_back()
    }

    fn clear(&mut self) {
        self.elements.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack() {
        let char_stack = ResizableCharBuffer::new();
        assert_eq!(
            char_stack.len(),
            0,
            "New stack should have length of 0 got {}",
            char_stack.len()
        );
        assert!(char_stack.is_empty());
    }

    #[test]
    fn is_empty() {
        let mut char_stack = ResizableCharBuffer::new();
        assert!(char_stack.is_empty());
        char_stack.push('a');
        assert!(!char_stack.is_empty());
    }

    #[test]
    fn top() {
        let mut char_stack = ResizableCharBuffer::new();
        assert_eq!(
            char_stack.top(),
            None,
            "The stack is empty, should get None, got {:?}",
            char_stack.top()
        );
        char_stack.push('a');
        char_stack.push('b');
        char_stack.push('c');
        assert_eq!(
            char_stack.len(),
            3,
            "Pushed 3 element, length should be 3. Got {}",
            char_stack.len()
        );
    }

    #[test]
    fn push() {
        let mut char_stack = ResizableCharBuffer::new();
        char_stack.push('a');
        assert_eq!(
            char_stack.len(),
            1,
            "Pushed 1 element, length should be 1. Got {}",
            char_stack.len()
        );
        char_stack.push('a');
        assert_eq!(
            char_stack.len(),
            2,
            "Pushed 2 element, length should be 2. Got {}",
            char_stack.len()
        );
        char_stack.push('a');
        assert_eq!(
            char_stack.len(),
            3,
            "Pushed 3 element, length should be 3. Got {}",
            char_stack.len()
        );
    }

    #[test]
    fn pop() {
        let mut char_stack = ResizableCharBuffer::new();
        assert_eq!(
            char_stack.pop(),
            None,
            "New stack should be empty, got {:?}",
            char_stack.pop()
        );
        char_stack.push('a');
        char_stack.push('b');
        char_stack.push('c');
        assert_eq!(char_stack.pop(), Some('c'));
        assert_eq!(char_stack.pop(), Some('b'));
        assert_eq!(char_stack.pop(), Some('a'));
        assert_eq!(char_stack.pop(), None);
    }

    #[test]
    fn clear() {
        let mut char_stack = ResizableCharBuffer::new();
        char_stack.push('a');
        char_stack.push('b');
        char_stack.push('c');
        char_stack.push('d');
        char_stack.push('e');
        char_stack.push('f');
        assert_eq!(char_stack.len(), 6);
        assert!(!char_stack.is_empty());
        char_stack.clear();
        assert_eq!(char_stack.len(), 0);
        assert!(char_stack.is_empty());
    }

    #[test]
    fn mix_usage() {
        let mut char_stack = ResizableCharBuffer::new();
        assert!(char_stack.is_empty());
        assert_eq!(char_stack.top(), None);
        assert_eq!(char_stack.pop(), None);
        char_stack.push('a');
        assert_eq!(char_stack.len(), 1);
        assert_eq!(char_stack.top(), Some(&'a'));
        assert_eq!(char_stack.pop(), Some('a'));
        char_stack.push('b');
        assert_eq!(char_stack.len(), 1);
        char_stack.push('c');
        assert_eq!(char_stack.len(), 2);
        assert_eq!(char_stack.pop(), Some('c'));
        assert_eq!(char_stack.len(), 1);
        assert_eq!(char_stack.top(), Some(&'b'));
        assert_eq!(char_stack.pop(), Some('b'));
        assert_eq!(char_stack.top(), None);
        assert_eq!(char_stack.pop(), None);
        assert_eq!(char_stack.len(), 0);
    }
}
