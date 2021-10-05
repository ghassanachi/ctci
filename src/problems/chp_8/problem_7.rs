pub fn permutation_with_dups<T: AsRef<str>>(input: T) -> Vec<String> {
    let input = input.as_ref();

    if input.len() == 0 {
        return Vec::new();
    }

    if input.len() == 1 {
        return vec![input.chars().nth(0).unwrap().to_string()];
    }

    let mut permutations: Vec<String> = Vec::new();
    for (i, cur_char) in input.chars().enumerate() {
        let without_cur = format!("{}{}", input[..i].to_string(), input[i + 1..].to_string());
        permutation_with_dups(without_cur)
            .drain(..)
            .for_each(|mut perm| {
                perm.push(cur_char);
                permutations.push(perm);
            })
    }
    permutations
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn permutation_with_dups_1() {
        let mut permutations = permutation_with_dups("tes");
        permutations.sort();
        let mut expected = vec!["tes", "tse", "ets", "est", "set", "ste"];
        expected.sort();
        assert_eq!(permutations, expected);
    }

    #[test]
    fn permutation_with_dups_2() {
        let permutations = permutation_with_dups("");
        let set: HashSet<&String> = HashSet::from_iter(&permutations);
        assert_eq!(permutations.len(), 0);
        assert_eq!(set.len(), permutations.len());
    }

    #[test]
    fn permutation_with_dups_3() {
        let permutations = permutation_with_dups("tesx");
        let set: HashSet<&String> = HashSet::from_iter(&permutations);
        assert_eq!(permutations.len(), 24);
        assert_eq!(set.len(), permutations.len());
    }

    #[test]
    fn permutation_with_dups_4() {
        let permutations = permutation_with_dups("sampling");
        let set: HashSet<&String> = HashSet::from_iter(&permutations);
        assert_eq!(permutations.len(), 40320);
        assert_eq!(set.len(), permutations.len());
    }
}
