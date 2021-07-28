use std::collections::HashSet;
pub fn all_chars_unique_part_a(s: &str) -> bool {
    let mut char_set: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if char_set.contains(&c) {
            return false;
        }
        char_set.insert(c);
    }
    true
}

pub fn all_chars_unique_part_b(s: &str) -> bool {
    let mut bit_map: i32 = 0;
    let base_char: u32 = 'a' as u32;

    for c in s.chars() {
        let mut int_char: u32 = c as u32;
        int_char -= base_char;

        if bit_map & (1 << int_char) != 0 {
            return false;
        }

        // set bit
        bit_map |= 1 << int_char;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert!(all_chars_unique_part_a(&String::from("abcdefg")));
        assert!(!all_chars_unique_part_a(&String::from("abcdefga")));
    }

    #[test]
    fn test_part_b() {
        assert!(all_chars_unique_part_b(&String::from("abcdefg")));
        assert!(!all_chars_unique_part_b(&String::from("abcdefga")));
    }
}
