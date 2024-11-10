use super::char_buffer::*;

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct KeyCombination(char, char);

#[derive(PartialEq, Debug)]
pub struct CombinationTarget {
    first: char,
    second: Option<char>,
}

impl CombinationTarget {
    pub fn is_reverse(&self) -> bool {
        self.second.is_some()
    }
}

pub type KeyCombinationMap = HashMap<KeyCombination, CombinationTarget>;

#[rustfmt::skip]
pub fn setup_key_combination_map() -> KeyCombinationMap {
    let mut char_map = KeyCombinationMap::new();

    // Combination map
    char_map.insert(KeyCombination('a', 'a'), CombinationTarget { first: 'â', second: None });
    char_map.insert(KeyCombination('e', 'e'), CombinationTarget { first: 'ê', second: None });
    char_map.insert(KeyCombination('i', 'i'), CombinationTarget { first: 'î', second: None });
    char_map.insert(KeyCombination('o', 'o'), CombinationTarget { first: 'ô', second: None });
    char_map.insert(KeyCombination('u', 'u'), CombinationTarget { first: 'û', second: None });
    
    char_map.insert(KeyCombination('a', 'f'), CombinationTarget { first: 'à', second: None });
    char_map.insert(KeyCombination('e', 'f'), CombinationTarget { first: 'è', second: None });
    char_map.insert(KeyCombination('u', 'f'), CombinationTarget { first: 'ù', second: None });
    
    char_map.insert(KeyCombination('e', 'x'), CombinationTarget { first: 'ë', second: None });
    char_map.insert(KeyCombination('i', 'x'), CombinationTarget { first: 'ï', second: None });
    char_map.insert(KeyCombination('u', 'x'), CombinationTarget { first: 'ü', second: None });
    
    char_map.insert(KeyCombination('e', 'w'), CombinationTarget { first: 'é', second: None });
    
    char_map.insert(KeyCombination('c', 'c'), CombinationTarget { first: 'ç', second: None });
    
    char_map.insert(KeyCombination('a', 'e'), CombinationTarget { first: 'æ', second: None });
    char_map.insert(KeyCombination('o', 'e'), CombinationTarget { first: 'œ', second: None });

    // Reverse combination map
    char_map.insert(KeyCombination('â', 'a'), CombinationTarget { first: 'a', second: Some('a') });
    char_map.insert(KeyCombination('ê', 'e'), CombinationTarget { first: 'e', second: Some('e') });
    char_map.insert(KeyCombination('î', 'i'), CombinationTarget { first: 'i', second: Some('i') });
    char_map.insert(KeyCombination('ô', 'o'), CombinationTarget { first: 'o', second: Some('o') });
    char_map.insert(KeyCombination('û', 'u'), CombinationTarget { first: 'u', second: Some('u') });
    
    char_map.insert(KeyCombination('à', 'f'), CombinationTarget { first: 'a', second: Some('f') });
    char_map.insert(KeyCombination('è', 'f'), CombinationTarget { first: 'e', second: Some('f') });
    char_map.insert(KeyCombination('ù', 'f'), CombinationTarget { first: 'u', second: Some('f') });
    
    char_map.insert(KeyCombination('ë', 'x'), CombinationTarget { first: 'e', second: Some('x') });
    char_map.insert(KeyCombination('ï', 'x'), CombinationTarget { first: 'i', second: Some('x') });
    char_map.insert(KeyCombination('ü', 'x'), CombinationTarget { first: 'u', second: Some('x') });
    
    char_map.insert(KeyCombination('é', 'w'), CombinationTarget { first: 'e', second: Some('w') });
    
    char_map.insert(KeyCombination('ç', 'c'), CombinationTarget { first: 'c', second: Some('c') });
    
    char_map.insert(KeyCombination('æ', 'e'), CombinationTarget { first: 'a', second: Some('e') });
    char_map.insert(KeyCombination('œ', 'e'), CombinationTarget { first: 'o', second: Some('e') });

    char_map
}

pub struct InputController<T: CharBuffer> {
    char_buffer: T,
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
        let previous_char = match self.char_buffer.top() {
            Some(c) => *c,
            None => {
                self.char_buffer.push(current_char);
                return None;
            }
        };

        let combination_target = match self
            .combination_map
            .get(&KeyCombination(previous_char, current_char))
        {
            Some(c) => c,
            None => {
                self.char_buffer.push(current_char);
                return None;
            }
        };

        self.char_buffer.pop();
        self.char_buffer.push(combination_target.first);
        if combination_target.is_reverse() {
            self.char_buffer.push(combination_target.second.unwrap());
        }

        println!("{:?}", self.char_buffer);

        Some(combination_target)
    }

    pub fn clear_char_buffer(&mut self) {
        self.char_buffer.clear();
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

            if combination_target.is_reverse() {
                assert_eq!(controller.char_buffer.len(), 2);
                assert_eq!(
                    controller.char_buffer.pop(),
                    Some(combination_target.second.unwrap())
                );
                assert_eq!(controller.char_buffer.pop(), Some(combination_target.first));
            } else {
                assert_eq!(controller.char_buffer.len(), 1);
                assert_eq!(controller.char_buffer.pop(), Some(combination_target.first));
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
