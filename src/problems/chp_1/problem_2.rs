use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut mappings = HashMap::new();
    for c in s1.chars() {
        *mappings.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        match mappings.entry(c) {
            Entry::Vacant(_) => return false,
            Entry::Occupied(o) => {
                let value = *o.get();
                if value == 1 {
                    mappings.remove(&c);
                } else {
                    mappings.insert(c, value - 1);
                }
            }
        }
    }
    mappings.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert!(is_permutation("testing", "testign"));
        assert!(!is_permutation("test", "testing"));
    }
}
