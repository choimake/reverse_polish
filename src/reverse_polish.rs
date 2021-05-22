/// Return the value calculated in reverse polish.
///
/// # Arguments
///
/// * `formula` - calculation formula
///
pub fn eval(formula: &str) -> i32 {
    let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
    eval_inner(&mut tokens)
}

fn eval_inner(tokens: &mut Vec<&str>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    let mut pos = 0;

    while let Some(token) = tokens.pop() {
        pos += 1;
        if let Ok(v) = token.parse::<i32>() {
            stack.push(v);
        } else {
            let rhs = stack
                .pop()
                .unwrap_or_else(|| panic!("invalid syntax at {}", pos));
            let lhs = stack
                .pop()
                .unwrap_or_else(|| panic!("invalid syntax at {}", pos));

            let result = match token {
                "+" => lhs + rhs,
                "-" => lhs - rhs,
                "*" => lhs * rhs,
                "/" => lhs / rhs,
                "%" => lhs % rhs,
                _ => panic!("invalid operator at {}", pos),
            };
            stack.push(result);
        }
    }
    if stack.len() == 1 {
        stack[0]
    } else {
        panic!("invalid syntax")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let formula = "1 2 +";
        assert_eq!(eval(formula), 3);
    }
    #[test]
    fn test_sub() {
        let formula = "1 2 -";
        assert_eq!(eval(formula), -1);
    }
    #[test]
    fn test_mul() {
        let formula = "1 2 *";
        assert_eq!(eval(formula), 2);
    }
    #[test]
    fn test_div() {
        let formula = "5 2 /";
        assert_eq!(eval(formula), 2);
    }
    #[test]
    fn test_mod() {
        let formula = "3 2 %";
        assert_eq!(eval(formula), 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_operator() {
        let formula = "1 1 _";
        assert_eq!(eval(formula), 1);
    }

    #[test]
    #[should_panic]
    fn test_fail_to_unwrap_value() {
        let formula = "a 1 +";
        assert_eq!(eval(formula), 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_calculation_formula() {
        let formula = "1 3";
        assert_eq!(eval(formula), 1);
    }
}
