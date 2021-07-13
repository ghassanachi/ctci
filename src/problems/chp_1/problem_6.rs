pub fn string_compression(s1: &str) -> String {
    let mut builder = String::new();
    let mut last_char: Option<(char, usize)> = None;

    let mut add_string = |opt: &Option<(char, usize)>| {
        if let Some((last_c, count)) = opt {
            if *count >= 2 {
                builder.push_str(&(count.to_string() + &last_c.to_string()));
            } else {
                builder.push_str(&last_c.to_string().repeat(*count))
            }
        }
    };

    for c in s1.chars() {
        if let Some((last_c, count)) = last_char.or(Some((c, 0))) {
            if last_c != c {
                add_string(&last_char);
                last_char = Some((c, 1));
            } else {
                last_char = Some((last_c, count + 1));
            }
        }
    }
    add_string(&last_char);
    if builder.len() >= s1.len() {
        return s1.to_string();
    }
    builder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_compression_1() {
        assert_eq!("h4e3a2ven", string_compression("heeeeaaavven"));
    }

    #[test]
    fn test_string_compression_2() {
        assert_eq!("baake", string_compression("baake"));
    }

    #[test]
    fn test_string_compression_3() {
        assert_eq!("10a", string_compression("aaaaaaaaaa"));
    }
}
