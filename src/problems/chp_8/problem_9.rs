pub fn valid_parens(n: u32) -> Vec<String> {
    valid_parens_helper(&mut String::new(), n, n)
}

fn valid_parens_helper(prefix: &mut String, open: u32, close: u32) -> Vec<String> {
    if open == 0 && close == 0 {
        return vec![prefix.clone()];
    }
    let mut result = Vec::new();
    if open > 0 {
        prefix.push('(');
        result.append(&mut valid_parens_helper(prefix, open - 1, close));
        prefix.pop();
    }
    if close > 0 && close > open {
        prefix.push(')');
        result.append(&mut valid_parens_helper(prefix, open, close - 1));
        prefix.pop();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_parens_1() {
        let result = valid_parens(0);
        let expected = vec![""];
        assert_eq!(result, expected);

        let result = valid_parens(1);
        let expected = vec!["()"];
        assert_eq!(result, expected);
    }

    #[test]
    fn valid_parens_2() {
        let mut result = valid_parens(2);
        result.sort();
        let mut expected = vec!["()()", "(())"];
        expected.sort();
        assert_eq!(result, expected);

        let mut result = valid_parens(3);
        result.sort();
        let mut expected = vec!["()()()", "(()())", "()(())", "(())()", "((()))"];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn valid_parens_3() {
        let result = valid_parens(7).len();
        assert_eq!(result, 429);

        let result = valid_parens(8).len();
        assert_eq!(result, 1_430);

        let result = valid_parens(10).len();
        assert_eq!(result, 16_796);
    }
}
