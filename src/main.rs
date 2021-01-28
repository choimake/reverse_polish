use std::env;

// 逆ポーランド記法の数式を計算するプログラム
// - 演算子は + - * / %
// - 数値は32bit整数

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}

fn reverse_polish(tokens: &mut Vec<&str>) -> i32 {
    tokens.reverse();

    let mut stack: Vec<i32> = Vec::new();

    while let Some(token) = tokens.pop() {
        if let Ok(v) = token.parse::<i32>() {
            stack.push(v);
        } else {
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();

            let result = match token {
                "+" => lhs + rhs,
                "-" => lhs - rhs,
                "*" => lhs * rhs,
                "/" => lhs / rhs,
                "%" => lhs % rhs,
                _ => panic!(),
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
        assert_eq!(reverse_polish(&mut v), 3)
    }
    #[test]
    fn test_sub() {
        let mut v: Vec<&str> = vec!["1", "2", "-"];
        assert_eq!(reverse_polish(&mut v), -1)
    }
    #[test]
    fn test_mul() {
        let mut v: Vec<&str> = vec!["1", "2", "*"];
        assert_eq!(reverse_polish(&mut v), 2)
    }
    #[test]
    fn test_div() {
        let mut v: Vec<&str> = vec!["5", "2", "/"];
        assert_eq!(reverse_polish(&mut v), 2)
    }
    #[test]
    fn test_mod() {
        let mut v: Vec<&str> = vec!["3", "2", "%"];
        assert_eq!(reverse_polish(&mut v), 1)
    }
}
