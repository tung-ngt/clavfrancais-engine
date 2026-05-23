use std::fmt::Debug;

pub trait CharBuffer: Debug {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn top(&self) -> Option<char>;

    fn push(&mut self, element: char);

    fn pop(&mut self) -> Option<char>;

    fn clear(&mut self);

    fn push_str(&mut self, text: &str);
}

mod heap_sized_char_buffer;
mod resizable_char_buffer;
mod stack_sized_char_buffer;

pub use heap_sized_char_buffer::HeapSizedCharBuffer;
pub use resizable_char_buffer::ResizableCharBuffer;
pub use stack_sized_char_buffer::StackSizedCharBuffer;
