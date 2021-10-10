use std::collections::HashMap;

pub fn bool_paren(expr: &str, eval: bool) -> Result<u32, &'static str> {
    let mut memo = HashMap::new();
    Ok(bool_paren_helper(expr, eval, &mut memo)?)
}

fn bool_paren_helper<'a>(
    expr: &'a str,
    eval: bool,
    memo: &mut HashMap<(bool, &'a str), u32>,
) -> Result<u32, &'static str> {
    if expr.len() % 2 != 1 {
        return Err("invalid expression");
    };
    if expr.len() == 0 {
        return Ok(0);
    }
    if expr.len() == 1 {
        let a: bool = match expr[0..1].parse::<u32>() {
            Ok(val) if val <= 1 => val == 1,
            _ => return Err("invalid bit"),
        };
        return if a == eval { Ok(1) } else { Ok(0) };
    }

    let key = (eval, expr);
    if memo.contains_key(&key) {
        return Ok(*memo.get(&key).unwrap());
    }

    let mut ways = 0;
    for i in (1..expr.len()).step_by(2) {
        let left = &expr[..i];
        let right = &expr[i + 1..];

        let left_true = bool_paren(&left, true)?;
        let left_false = bool_paren(&left, false)?;
        let right_true = bool_paren(&right, true)?;
        let right_false = bool_paren(&right, false)?;

        let total = (left_true + left_false) * (right_true + right_false);
        let cur_ways = match &expr[i..i + 1] {
            "|" => left_true * right_false + left_false * right_true + left_true * right_true,
            "^" => left_false * right_true + left_true * right_false,
            "&" => left_true * right_true,
            _ => return Err("invalid operator"),
        };
        ways += if eval { cur_ways } else { total - cur_ways };
    }
    *memo.entry(key).or_insert(0) = ways;
    Ok(ways)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_paren_1() {
        assert_eq!(bool_paren("1^0|0|1", false), Ok(2));
        assert_eq!(bool_paren("0&0&0&1^1|0", true), Ok(10));
    }

    #[test]
    fn boolean_paren_2() {
        assert_eq!(bool_paren("10|0|1", true), Err("invalid expression"));
        assert_eq!(bool_paren("1&0*0|1", false), Err("invalid operator"));
        assert_eq!(bool_paren("1&0|0|2", false), Err("invalid bit"));
    }
}
