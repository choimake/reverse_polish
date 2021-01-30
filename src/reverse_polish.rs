/// Return the value calculated in reverse polish.
///
/// # Arguments
///
/// * `tokens` - calculation formula
///
pub fn eval(tokens: &mut Vec<&str>) -> i32 {
    tokens.reverse();

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
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut v: Vec<&str> = vec!["1", "2", "+"];
        assert_eq!(eval(&mut v), 3)
    }
    #[test]
    fn test_sub() {
        let mut v: Vec<&str> = vec!["1", "2", "-"];
        assert_eq!(eval(&mut v), -1)
    }
    #[test]
    fn test_mul() {
        let mut v: Vec<&str> = vec!["1", "2", "*"];
        assert_eq!(eval(&mut v), 2)
    }
    #[test]
    fn test_div() {
        let mut v: Vec<&str> = vec!["5", "2", "/"];
        assert_eq!(eval(&mut v), 2)
    }
    #[test]
    fn test_mod() {
        let mut v: Vec<&str> = vec!["3", "2", "%"];
        assert_eq!(eval(&mut v), 1)
    }

    #[test]
    #[should_panic]
    fn test_invalid_operator() {
        let mut v: Vec<&str> = vec!["1", "1", "_"];
        assert_eq!(eval(&mut v), 1)
    }

    #[test]
    #[should_panic]
    fn test_fail_to_unwrap_value() {
        let mut v: Vec<&str> = vec!["a", "1", "+"];
        assert_eq!(eval(&mut v), 1)
    }
}
