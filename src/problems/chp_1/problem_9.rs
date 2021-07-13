pub fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    s2.repeat(2).contains(s1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rotation_1() {
        assert!(is_rotation("waterbottle", "erbottlewat"))
    }

    #[test]
    fn test_is_rotation_2() {
        assert!(!is_rotation("waterbottle", "erbottlat"))
    }

    #[test]
    fn test_is_rotation_3() {
        assert!(is_rotation("aaa", "aaa"))
    }
}
