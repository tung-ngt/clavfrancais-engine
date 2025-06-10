use crate::debug_println;

use super::char_buffer::*;

use std::{collections::HashMap, hint::unreachable_unchecked};

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum KeyCombination {
    Single(char),
    Double(char, char),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CombinationTarget {
    Replace(char),
    Combine(char),
    Revert(char, char),
}

pub type KeyCombinationMap = HashMap<KeyCombination, CombinationTarget>;

#[rustfmt::skip]
pub fn setup_key_combination_map() -> KeyCombinationMap {
    let mut char_map = KeyCombinationMap::new();

    // Replace map
    char_map.insert(KeyCombination::Single('$'), CombinationTarget::Replace('€'));
    char_map.insert(KeyCombination::Single('<'), CombinationTarget::Replace('«'));
    char_map.insert(KeyCombination::Single('>'), CombinationTarget::Replace('»'));

    // Reverse replace map
    char_map.insert(KeyCombination::Double('€', '€'), CombinationTarget::Combine('$'));
    char_map.insert(KeyCombination::Double('«', '«'), CombinationTarget::Combine('<'));
    char_map.insert(KeyCombination::Double('»', '»'), CombinationTarget::Combine('>'));

    // Combination map
    char_map.insert(KeyCombination::Double('a', 'a'), CombinationTarget::Combine('â'));
    char_map.insert(KeyCombination::Double('e', 'e'), CombinationTarget::Combine('ê'));
    char_map.insert(KeyCombination::Double('i', 'i'), CombinationTarget::Combine('î'));
    char_map.insert(KeyCombination::Double('o', 'o'), CombinationTarget::Combine('ô'));
    char_map.insert(KeyCombination::Double('u', 'u'), CombinationTarget::Combine('û'));
    
    char_map.insert(KeyCombination::Double('a', 'f'), CombinationTarget::Combine('à'));
    char_map.insert(KeyCombination::Double('e', 'f'), CombinationTarget::Combine('è'));
    char_map.insert(KeyCombination::Double('u', 'f'), CombinationTarget::Combine('ù'));
    
    char_map.insert(KeyCombination::Double('e', 'x'), CombinationTarget::Combine('ë'));
    char_map.insert(KeyCombination::Double('i', 'x'), CombinationTarget::Combine('ï'));
    char_map.insert(KeyCombination::Double('u', 'x'), CombinationTarget::Combine('ü'));
    
    char_map.insert(KeyCombination::Double('e', 'w'), CombinationTarget::Combine('é'));
    
    char_map.insert(KeyCombination::Double('c', 'c'), CombinationTarget::Combine('ç'));
    
    char_map.insert(KeyCombination::Double('a', 'e'), CombinationTarget::Combine('æ'));
    char_map.insert(KeyCombination::Double('o', 'e'), CombinationTarget::Combine('œ'));

    // Reverse combination map
    char_map.insert(KeyCombination::Double('â', 'a'), CombinationTarget::Revert('a', 'a'));
    char_map.insert(KeyCombination::Double('ê', 'e'), CombinationTarget::Revert('e', 'e'));
    
    char_map.insert(KeyCombination::Double('î', 'i'), CombinationTarget::Revert('i', 'i'));
    char_map.insert(KeyCombination::Double('ô', 'o'), CombinationTarget::Revert('o', 'o'));
    char_map.insert(KeyCombination::Double('û', 'u'), CombinationTarget::Revert('u', 'u'));
    
    char_map.insert(KeyCombination::Double('à', 'f'), CombinationTarget::Revert('a', 'f'));
    char_map.insert(KeyCombination::Double('è', 'f'), CombinationTarget::Revert('e', 'f'));
    char_map.insert(KeyCombination::Double('ù', 'f'), CombinationTarget::Revert('u', 'f'));
    
    char_map.insert(KeyCombination::Double('ë', 'x'), CombinationTarget::Revert('e', 'x'));
    char_map.insert(KeyCombination::Double('ï', 'x'), CombinationTarget::Revert('i', 'x'));
    char_map.insert(KeyCombination::Double('ü', 'x'), CombinationTarget::Revert('u', 'x'));
    
    char_map.insert(KeyCombination::Double('é', 'w'), CombinationTarget::Revert('e', 'w'));

    char_map.insert(KeyCombination::Double('ç', 'c'), CombinationTarget::Revert('c', 'c'));
    
    char_map.insert(KeyCombination::Double('æ', 'e'), CombinationTarget::Revert('a', 'e'));
    char_map.insert(KeyCombination::Double('œ', 'e'), CombinationTarget::Revert('o', 'e'));

    char_map
}
#[derive(Debug)]
pub struct Engine<T: CharBuffer> {
    pub char_buffer: T,
    combination_map: KeyCombinationMap,
}

impl<T: CharBuffer> Engine<T> {
    pub fn new(combination_map: KeyCombinationMap, char_buffer: T) -> Self {
        Self {
            char_buffer,
            combination_map,
        }
    }

    pub fn add_char(&mut self, current_char: char) -> Option<CombinationTarget> {
        debug_println!("{:?}", self.char_buffer);
        let current_char_lower = current_char.to_lowercase().next().unwrap();
        if let Some(combination_target) = self
            .combination_map
            .get(&KeyCombination::Single(current_char_lower))
        {
            let CombinationTarget::Replace(c) = combination_target else {
                unsafe {
                    unreachable_unchecked();
                }
            };

            self.char_buffer.push(*c);
            return Some(*combination_target);
        }

        let Some(previous_char) = self.char_buffer.top() else {
            self.char_buffer.push(current_char);
            return None;
        };

        let previous_char_lower = previous_char.to_lowercase().next().unwrap();

        let Some(combination_target) = self.combination_map.get(&KeyCombination::Double(
            previous_char_lower,
            current_char_lower,
        )) else {
            self.char_buffer.push(current_char);
            return None;
        };

        self.char_buffer.pop();

        let combination_target = match combination_target {
            CombinationTarget::Combine(f) => {
                let f = if previous_char.is_lowercase() {
                    *f
                } else {
                    f.to_uppercase().next().unwrap()
                };

                self.char_buffer.push(f);
                CombinationTarget::Combine(f)
            }
            CombinationTarget::Revert(f, s) => {
                let f = if previous_char.is_lowercase() {
                    *f
                } else {
                    f.to_uppercase().next().unwrap()
                };

                let s = if current_char.is_lowercase() {
                    *s
                } else {
                    s.to_uppercase().next().unwrap()
                };

                self.char_buffer.push(f);
                self.char_buffer.push(s);

                CombinationTarget::Revert(f, s)
            }
            _ => unsafe {
                unreachable_unchecked();
            },
        };

        Some(combination_target)
    }

    pub fn clear_char_buffer(&mut self) {
        self.char_buffer.clear();
    }

    pub fn backspace(&mut self) {
        self.char_buffer.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_new_controller() -> Engine<ResizableCharBuffer> {
        let key_combination_map = setup_key_combination_map();
        Engine::new(key_combination_map, ResizableCharBuffer::new())
    }

    #[test]
    fn new_controller() {
        let controller = setup_new_controller();
        let key_combination_map = setup_key_combination_map();
        assert!(controller.char_buffer.is_empty());
        assert_eq!(controller.combination_map, key_combination_map);
    }

    #[test]
    fn add_char() {
        let mut controller = setup_new_controller();
        let key_combination_map = setup_key_combination_map();
        assert!(controller.char_buffer.is_empty());

        for (key_combination, combination_target) in key_combination_map {
            match key_combination {
                KeyCombination::Single(c) => {
                    assert_eq!(controller.add_char(c), Some(combination_target));
                }
                KeyCombination::Double(f, s) => {
                    assert_eq!(controller.add_char(f), None);
                    assert_eq!(controller.add_char(s), Some(combination_target));

                    match combination_target {
                        CombinationTarget::Combine(_) => {
                            controller.char_buffer.pop();
                        }
                        CombinationTarget::Revert(_, _) => {
                            controller.char_buffer.pop();
                            controller.char_buffer.pop();
                        }
                        _ => unreachable!("should be combine or revert"),
                    }
                }
            }
            controller.clear_char_buffer();
        }
    }

    #[test]
    fn clear_char_buffer() {
        let mut controller = setup_new_controller();
        controller.add_char('1');
        controller.add_char('2');
        controller.add_char('3');
        controller.add_char(' ');
        controller.add_char('5');
        controller.add_char('6');
        controller.add_char('7');

        assert_eq!(controller.char_buffer.len(), 7);

        controller.clear_char_buffer();

        assert!(controller.char_buffer.is_empty());
    }
}
