use super::char_buffer::*;

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct KeyCombination(char, char);

#[derive(PartialEq, Debug)]
pub enum CombinationTarget {
    Combine(char),
    Revert(char, char),
}

pub type KeyCombinationMap = HashMap<KeyCombination, CombinationTarget>;

#[rustfmt::skip]
pub fn setup_key_combination_map() -> KeyCombinationMap {
    let mut char_map = KeyCombinationMap::new();

    // Combination map
    char_map.insert(KeyCombination('a', 'a'), CombinationTarget::Combine('â'));
    char_map.insert(KeyCombination('e', 'e'), CombinationTarget::Combine('ê'));
    char_map.insert(KeyCombination('i', 'i'), CombinationTarget::Combine('î'));
    char_map.insert(KeyCombination('o', 'o'), CombinationTarget::Combine('ô'));
    char_map.insert(KeyCombination('u', 'u'), CombinationTarget::Combine('û'));
    
    char_map.insert(KeyCombination('a', 'f'), CombinationTarget::Combine('à'));
    char_map.insert(KeyCombination('e', 'f'), CombinationTarget::Combine('è'));
    char_map.insert(KeyCombination('u', 'f'), CombinationTarget::Combine('ù'));
    
    char_map.insert(KeyCombination('e', 'x'), CombinationTarget::Combine('ë'));
    char_map.insert(KeyCombination('i', 'x'), CombinationTarget::Combine('ï'));
    char_map.insert(KeyCombination('u', 'x'), CombinationTarget::Combine('ü'));
    
    char_map.insert(KeyCombination('e', 'w'), CombinationTarget::Combine('é'));
    
    char_map.insert(KeyCombination('c', 'c'), CombinationTarget::Combine('ç'));
    
    char_map.insert(KeyCombination('a', 'e'), CombinationTarget::Combine('æ'));
    char_map.insert(KeyCombination('o', 'e'), CombinationTarget::Combine('œ'));

    // Reverse combination map
    char_map.insert(KeyCombination('â', 'a'), CombinationTarget::Revert('a', 'a'));
    char_map.insert(KeyCombination('ê', 'e'), CombinationTarget::Revert('e', 'e'));
    
    char_map.insert(KeyCombination('î', 'i'), CombinationTarget::Revert('i', 'i'));
    char_map.insert(KeyCombination('ô', 'o'), CombinationTarget::Revert('o', 'o'));
    char_map.insert(KeyCombination('û', 'u'), CombinationTarget::Revert('u', 'u'));
    
    char_map.insert(KeyCombination('à', 'f'), CombinationTarget::Revert('a', 'f'));
    char_map.insert(KeyCombination('è', 'f'), CombinationTarget::Revert('e', 'f'));
    char_map.insert(KeyCombination('ù', 'f'), CombinationTarget::Revert('u', 'f'));
    
    char_map.insert(KeyCombination('ë', 'x'), CombinationTarget::Revert('e', 'x'));
    char_map.insert(KeyCombination('ï', 'x'), CombinationTarget::Revert('i', 'x'));
    char_map.insert(KeyCombination('ü', 'x'), CombinationTarget::Revert('u', 'x'));
    
    char_map.insert(KeyCombination('é', 'w'), CombinationTarget::Revert('e', 'w'));

    char_map.insert(KeyCombination('ç', 'c'), CombinationTarget::Revert('c', 'c'));
    
    char_map.insert(KeyCombination('æ', 'e'), CombinationTarget::Revert('a', 'e'));
    char_map.insert(KeyCombination('œ', 'e'), CombinationTarget::Revert('o', 'e'));

    char_map
}
#[derive(Debug)]
pub struct InputController<T: CharBuffer> {
    pub char_buffer: T,
    combination_map: KeyCombinationMap,
}

impl<T: CharBuffer> InputController<T> {
    pub fn new(combination_map: KeyCombinationMap, char_buffer: T) -> Self {
        Self {
            char_buffer,
            combination_map,
        }
    }

    pub fn add_char(&mut self, current_char: char) -> Option<&CombinationTarget> {
        let Some(previous_char) = self.char_buffer.top()
        else {
            self.char_buffer.push(current_char);
            return None;
        };

        let Some(combination_target) = self.combination_map
            .get(&KeyCombination(previous_char, current_char))
        else {
            self.char_buffer.push(current_char);
            return None;
        };

        self.char_buffer.pop();

        match combination_target {
            CombinationTarget::Combine(f) => {
                self.char_buffer.push(*f);
            },
            CombinationTarget::Revert(f, s) => {
                self.char_buffer.push(*f);
                self.char_buffer.push(*s);
            },
        }

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

    fn setup_new_controller() -> InputController<ResizableCharBuffer> {
        let key_combination_map = setup_key_combination_map();
        InputController::new(key_combination_map, ResizableCharBuffer::new())
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
            assert_eq!(controller.add_char(key_combination.0), None);
            assert_eq!(
                controller.add_char(key_combination.1),
                Some(&combination_target)
            );

            match combination_target {
                CombinationTarget::Combine(_) => {
                    controller.char_buffer.pop();
                },
                CombinationTarget::Revert(_, _) => {
                    controller.char_buffer.pop();
                    controller.char_buffer.pop();
                },
            }
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
