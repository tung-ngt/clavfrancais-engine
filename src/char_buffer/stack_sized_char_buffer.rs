use super::CharBuffer;

#[derive(Debug)]
pub struct StackSizedCharBuffer<const CAPACITY: usize> {
    elements: [char; CAPACITY],
    top_index: usize,
    len: usize,
}

impl<const CAPACITY: usize> Default for StackSizedCharBuffer<CAPACITY> {
    fn default() -> Self {
        assert!(CAPACITY != 0, "capacity must be > 0");
        Self {
            elements: ['\0'; CAPACITY],
            top_index: 0,
            len: 0,
        }
    }
}

impl<const CAPACITY: usize> StackSizedCharBuffer<CAPACITY> {
    pub fn is_full(&self) -> bool {
        self.len == CAPACITY
    }
}

impl<const CAPACITY: usize> CharBuffer for StackSizedCharBuffer<CAPACITY> {
    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn top(&self) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            Some(self.elements[self.top_index])
        }
    }

    fn push(&mut self, element: char) {
        self.top_index = (self.top_index + 1) % CAPACITY;
        self.len += if self.is_full() { 0 } else { 1 };
        self.elements[self.top_index] = element;
    }

    fn pop(&mut self) -> Option<char> {
        if self.is_empty() {
            return None;
        }
        let top_element = self.elements[self.top_index];

        self.top_index = (self.top_index + CAPACITY - 1) % CAPACITY;
        self.len -= 1;
        Some(top_element)
    }

    fn clear(&mut self) {
        self.len = 0;
    }
}
