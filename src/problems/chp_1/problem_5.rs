use std::cmp::Ordering;

fn has_one_edit(s1: &str, s2: &str) -> bool {
    assert!((s1.len() as i32 - s2.len() as i32).abs() <= 1);

    let mut has_edit = false;
    let mut s1_iter = s1.chars();
    let mut s2_iter = s2.chars();
    let mut c1_opt = s1_iter.next();
    let mut c2_opt = s2_iter.next();

    loop {
        match (c1_opt, c2_opt) {
            (Some(c1), Some(c2)) => {
                if c1 == c2 {
                    c1_opt = s1_iter.next();
                    c2_opt = s2_iter.next();
                } else {
                    if has_edit {
                        return false;
                    }
                    has_edit = true;
                    match s1.len().cmp(&s2.len()) {
                        Ordering::Greater => {
                            c1_opt = s1_iter.next();
                        }
                        Ordering::Less => {
                            c2_opt = s2_iter.next();
                        }
                        Ordering::Equal => {
                            c1_opt = s1_iter.next();
                            c2_opt = s2_iter.next();
                        }
                    }
                }
            }
            (Some(_), None) => return !has_edit && s1.len() < s2.len(),
            (None, Some(_)) => return !has_edit && s1.len() < s2.len(),
            _ => return true,
        }
    }
}

pub fn one_away(s1: &str, s2: &str) -> bool {
    if s1 == s2 {
        return true;
    }
    if (s1.len() as i32 - s2.len() as i32).abs() > 1 {
        return false;
    }
    has_one_edit(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_away_1() {
        assert!(one_away("tea", "tee"));
    }

    #[test]
    fn test_one_away_2() {
        assert!(one_away("bake", "brake"));
    }

    #[test]
    fn test_one_away_3() {
        assert!(!one_away("pace", "bake"));
    }
}
