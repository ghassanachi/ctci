pub fn urlify(input: &mut Vec<char>, true_len: usize) {
    let mut space_count: usize = 0;
    for (i, c) in input.iter().enumerate() {
        if i == true_len {
            break;
        }
        if *c == ' ' {
            space_count += 1
        }
    }
    let mut runner: usize = true_len + (space_count * 2);
    for i in (0..true_len).rev() {
        if input[i] == ' ' {
            input[runner - 1] = '0';
            input[runner - 2] = '2';
            input[runner - 3] = '%';
            runner -= 3;
        } else {
            input[runner - 1] = input[i];
            runner -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urlify_test_1() {
        let mut input = String::from("Mr John Smith    ").chars().collect();
        urlify(&mut input, 13);
        assert_eq!("Mr%20John%20Smith", input.into_iter().collect::<String>())
    }

    #[test]
    fn urlify_test_2() {
        let mut input = String::from("Mr John Smith       ").chars().collect();
        urlify(&mut input, 14);
        assert_eq!(
            "Mr%20John%20Smith%20",
            input.into_iter().collect::<String>()
        )
    }

    #[test]
    fn urlify_test_3() {
        let mut input = String::from(" Mr John Smith      ").chars().collect();
        urlify(&mut input, 14);
        assert_eq!(
            "%20Mr%20John%20Smith",
            input.into_iter().collect::<String>()
        )
    }
}
