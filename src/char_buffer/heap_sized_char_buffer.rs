use super::CharBuffer;

#[derive(Debug)]
pub struct HeapSizedCharBuffer {
    elements: Box<[char]>,
    capacity: usize,
    top_index: usize,
    len: usize,
}

impl HeapSizedCharBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            elements: vec!['\0'; capacity].into_boxed_slice(),
            top_index: 0,
            len: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}

impl CharBuffer for HeapSizedCharBuffer {
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
        self.top_index = (self.top_index + 1) % self.capacity;
        self.len += if self.is_full() { 0 } else { 1 };
        self.elements[self.top_index] = element;
    }

    fn pop(&mut self) -> Option<char> {
        if self.is_empty() {
            return None;
        }
        let top_element = self.elements[self.top_index];

        self.top_index = (self.top_index + self.capacity - 1) % self.capacity;
        self.len -= 1;
        Some(top_element)
    }

    fn clear(&mut self) {
        self.len = 0;
    }
}
