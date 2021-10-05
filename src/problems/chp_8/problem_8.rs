use std::collections::{hash_map::Entry, HashMap};

fn get_counts<T: AsRef<str>>(input: T) -> HashMap<char, u32> {
    let mut counts = HashMap::new();
    for c in input.as_ref().chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

pub fn permutation_without_dups<T: AsRef<str>>(input: T) -> Vec<String> {
    let mut counts = get_counts(input);
    let mut permutations = Vec::new();
    permutation_without_dups_helper(&mut String::new(), &mut counts, &mut permutations);
    permutations
}

fn permutation_without_dups_helper(
    prefix: &mut String,
    counts: &mut HashMap<char, u32>,
    permutations: &mut Vec<String>,
) {
    if counts.is_empty() && prefix.len() != 0 {
        return permutations.push(prefix.clone());
    }
    let mappings: Vec<(char, u32)> = counts.iter().map(|(&key, &value)| (key, value)).collect();
    for (key, val) in mappings {
        match counts.entry(key) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() = val - 1;
                if *entry.get() == 0 {
                    entry.remove_entry();
                };
                prefix.push(key);
                permutation_without_dups_helper(prefix, counts, permutations);
                prefix.pop();
                *counts.entry(key).or_insert(0) = val;
            }
            Entry::Vacant(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn permutation_without_dups_1() {
        let mut permutations = permutation_without_dups("tes");
        permutations.sort();
        let mut expected = vec!["tes", "tse", "ets", "est", "set", "ste"];
        expected.sort();
        assert_eq!(permutations, expected);

        let mut permutations = permutation_without_dups("tts");
        permutations.sort();
        let mut expected = vec!["tts", "tst", "stt"];
        expected.sort();
        assert_eq!(permutations, expected);
    }

    #[test]
    fn permutation_without_dups_2() {
        let permutations = permutation_without_dups("");
        let set: HashSet<&String> = HashSet::from_iter(&permutations);
        assert_eq!(permutations.len(), 0);
        assert_eq!(set.len(), permutations.len());

        let mut permutations = permutation_without_dups("t".repeat(10));
        permutations.sort();
        let mut expected = vec!["t".repeat(10)];
        expected.sort();
        assert_eq!(permutations, expected);
    }

    #[test]
    fn permutation_without_dups_3() {
        let permutations = permutation_without_dups("tesx");
        let set: HashSet<&String> = HashSet::from_iter(&permutations);
        assert_eq!(permutations.len(), 24);
        assert_eq!(set.len(), permutations.len());

        let mut permutations = permutation_without_dups("tsts");
        permutations.sort();
        let mut expected = vec!["ttss", "stts", "sstt", "tsst", "tsts", "stst"];
        expected.sort();
        assert_eq!(permutations, expected);
    }
}
